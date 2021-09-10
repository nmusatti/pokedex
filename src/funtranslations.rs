use std::collections::HashMap;

use reqwest::{Client, Request};

use serde::Deserialize;

use crate::error::{Error, FuntranslationsError};

pub(crate) enum Language {
    Shakespeare,
    Yoda,
}

impl Language {
    pub(crate) fn lang(&self) -> &'static str {
        match self {
            Language::Shakespeare => "shakespeare",
            Language::Yoda => "yoda",
        }
    }
}

#[derive(Deserialize, Debug)]
struct Contents {
    translated: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Translation {
    contents: Option<Contents>,
}

fn make_request(client: &Client, text: &str, lang: &Language) -> Result<Request, reqwest::Error> {
    let map: HashMap<&str, &str> = [("text", text)].iter().cloned().collect();
    client
        .post(format!(
            "https://api.funtranslations.com/translate/{}.json",
            Language::lang(lang)
        ))
        .form(&map)
        .build()
}

pub(crate) async fn funtranslations(text: &str, lang: Language) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let trans_request = make_request(&client, text, &lang)?;
    let trans_resp = client.execute(trans_request).await;
    match trans_resp {
        Ok(resp) => {
            let translation = resp.json::<Translation>().await?;
            Ok(translation.contents.unwrap().translated.unwrap())
        }
        Err(err) => {
            let msg = if let Some(status) = err.status() {
                format!("Error code: {}", status.as_str())
            } else {
                "Unknown error".to_owned()
            };
            Err(FuntranslationsError::BadResponse(msg).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;

    use super::{funtranslations, make_request, Language};

    #[test]
    fn make_request_ok() -> Result<(), Error> {
        let client = reqwest::Client::new();
        let text = "Who does the urlencoding?";
        let req = make_request(&client, text, &Language::Shakespeare)?;
        let body = req.body().unwrap().as_bytes().unwrap();
        assert_eq!(body, text.as_bytes());
        Ok(())
    }

    #[tokio::test]
    async fn request_ok() {
        let url = format!(
            "https://api.funtranslations.com/translate/{}.json",
            Language::lang(&Language::Yoda)
        );
        assert_eq!(url, "https://api.funtranslations.com/translate/yoda.json");
        let client = reqwest::Client::new();
        let trans_resp = client.post(url).form("Jane skips rope").send().await;
        match trans_resp {
            Ok(resp) => {
                assert_eq!(resp.status().as_u16(), 200);
            }
            Err(err) => {
                if let Some(status) = err.status() {
                    assert_eq!(status.as_u16(), 400);
                }
            }
        };
    }

    #[tokio::test]
    async fn yoda_ok() -> Result<(), Error> {
        let resp = funtranslations("Jane skips rope", Language::Yoda).await?;
        assert_eq!("Rope,  jane skips", resp);
        Ok(())
    }
}
