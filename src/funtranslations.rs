use crate::error::Error;

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

pub(crate) async fn funtranslations(text: &str, lang: Language) -> Result<Option<String>, Error> {
    let client = reqwest::Client::new();
    let trans_resp = client.post(format!("https://api.funtranslations.com/translate/{}.json", Language::lang(&lang)))
        .form(text)
        .send()
        .await;
    match trans_resp {
        Ok(resp) => {
            let trans_json = resp.json::<serde_json::Value>().await?;
            let translation = trans_json["contents"].as_object().unwrap()["translated"].as_str().unwrap();
            Ok(Some(translation.to_owned()))
                },
        Err(_) => {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Language, funtranslations};

    #[tokio::test]
    async fn request_ok() {
        let url = format!("https://api.funtranslations.com/translate/{}.json", Language::lang(&Language::Yoda));
        assert_eq!(url, "https://api.funtranslations.com/translate/yoda.json");
        let client = reqwest::Client::new();
        let trans_resp = client.post(url)
            .form("Jane skips rope")
            .send()
            .await;
        match trans_resp {
            Ok(resp) => {
                assert_eq!(resp.status().as_u16(), 200);
            },
            Err(err) => {
                if let Some(status) = err.status() {
                    assert_eq!(status.as_u16(), 400);
                }
            },
        };
    }

    #[tokio::test]
    async fn yoda_ok() {
        let resp = funtranslations("Jane skips rope", Language::Yoda).await.unwrap();
        if let Some(trans) = resp {
            assert_eq!("Rope, jane  skips", trans);
        }
    }
}