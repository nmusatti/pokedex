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

pub(crate) async fn funtranslations(text: &str, lang: Language) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let trans_resp = client
        .post(format!(
            "https://api.funtranslations.com/translate/{}.json",
            Language::lang(&lang)
        ))
        .form(text)
        .send()
        .await;
    match trans_resp {
        Ok(resp) => {
            let trans_json = resp.json::<serde_json::Value>().await?;
            let translation = trans_json["contents"].as_object().unwrap()["translated"]
                .as_str()
                .unwrap();
            Ok(translation.to_owned())
        }
        Err(err) => {
            let msg = if let Some(status) = err.status() {
                format!("Error code: {}", status.as_str())
            }
            else {
                "Unknown error".to_owned()
            };
            Err(FuntranslationsError::BadResponse(msg).into())
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;

    use super::{funtranslations, Language};

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
        let resp = funtranslations("Jane skips rope", Language::Shakespeare)
            .await?;
        assert_eq!("Rope, jane  skips", resp);
        Ok(())
    }
}
