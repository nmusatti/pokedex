use std::collections::HashMap;

use async_trait::async_trait;

use reqwest::{Client, Request, Response};

use serde::Deserialize;

use crate::{
    error::{Error, HttpError},
    model::{Language, Translator},
};

#[derive(Deserialize, Debug)]
struct Contents {
    translated: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Translation {
    contents: Option<Contents>,
}

pub(crate) struct Funtranslations {
    client: Client,
}

impl Funtranslations {
    pub(crate) fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    fn make_request(&self, text: &str, lang: Language) -> Result<Request, reqwest::Error> {
        let map: HashMap<&str, &str> = [("text", text)].iter().cloned().collect();
        self.client
            .post(format!(
                "https://api.funtranslations.com/translate/{}.json",
                lang.to_string()
            ))
            .form(&map)
            .build()
    }

    async fn execute(&self, request: Request) -> Result<Response, reqwest::Error> {
        self.client.execute(request).await
    }

    pub(crate) async fn translate(&self, text: &str, lang: Language) -> Result<String, Error> {
        let trans_request = self.make_request(text, lang)?;
        let trans_resp = self.execute(trans_request).await;
        match trans_resp {
            Ok(resp) => {
                let translation = resp.json::<Translation>().await?;
                Ok(translation.contents.unwrap().translated.unwrap())
            }
            Err(err) => Err(HttpError::extract(err)),
        }
    }
}

#[async_trait]
impl Translator for Funtranslations {
    async fn translate(&self, text: &str, lang: Language) -> Result<String, Error> {
        self.translate(text, lang).await
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;

    use super::{Funtranslations, Language};

    #[test]
    fn make_request_encodes_text() -> Result<(), Error> {
        let translator = Funtranslations::new();
        let req = translator.make_request("Who does the urlencoding?", Language::Shakespeare)?;
        let body = req.body().unwrap().as_bytes().unwrap();
        assert_eq!(
            std::str::from_utf8(body),
            Ok("text=Who+does+the+urlencoding%3F")
        );
        Ok(())
    }

    #[tokio::test]
    async fn check_status_codes() -> Result<(), Error> {
        let translator = Funtranslations::new();
        let trans_request = translator.make_request("Jane skips rope", Language::Yoda)?;
        let trans_resp = translator.execute(trans_request).await;
        match trans_resp {
            Ok(resp) => assert_eq!(resp.status().as_u16(), 200),
            Err(err) => assert!(err.status().unwrap().as_u16() > 299),
        };
        Ok(())
    }

    #[tokio::test]
    async fn check_translation() {
        let translator = Funtranslations::new();
        let resp = translator
            .translate("Jane skips rope", Language::Yoda)
            .await;
        match resp {
            Ok(trans) => assert_eq!("Rope,  jane skips", trans),
            Err(err) => {
                if let Error::Http(http) = err {
                    assert!(http.status > 299);
                }
            },
        };
    }
}
