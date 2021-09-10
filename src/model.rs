use async_trait::async_trait;

use rocket::serde::Serialize;

use crate::error::Error;

#[derive(Serialize)]
pub(crate) struct Pokemon {
    name: String,
    description: String,
    habitat: String,
    is_legendary: bool,
}

impl Pokemon {
    pub(crate) fn new(name: &str, description: &str, habitat: &str, is_legendary: bool) -> Self {
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
            habitat: habitat.to_owned(),
            is_legendary,
        }
    }
}

pub(crate) enum Mode {
    Plain,
    Translated,
}

pub(crate) enum Language {
    Shakespeare,
    Yoda,
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Language::Shakespeare => "shakespeare".to_owned(),
            Language::Yoda => "yoda".to_owned(),
        }
    }
}

#[async_trait]
pub(crate) trait Translator: Send + Sync {
    async fn translate(& self, text: & str, lang: Language) -> Result<String, Error>;
}
