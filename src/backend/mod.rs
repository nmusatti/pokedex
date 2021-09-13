mod pokeapi;
mod funtranslations;

use async_trait::async_trait;

use crate::{error::Error, model::{Language, Mode, Pokemon}};

use self::{funtranslations::Funtranslations, pokeapi::Pokeapi};

#[async_trait]
pub(crate) trait Translator: Send + Sync {
    async fn translate(&self, text: &str, lang: Language) -> Result<String, Error>;
}

#[async_trait]
pub(crate) trait PokemonSource: Send + Sync {
    async fn pokemon(&self, name: &str, mode: Mode) -> Result<Pokemon, Error>;
}

pub(crate) fn backend() -> Box<dyn PokemonSource> {
    Box::new(Pokeapi::new(Box::new(Funtranslations::new())))
}
