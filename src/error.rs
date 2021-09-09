use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Funtranslations(#[from] FuntranslationsError),
    #[error(transparent)]
    Pokeapi(#[from] PokeapiError),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Rocket(#[from] rocket::Error),
}

#[derive(Error, Debug)]
pub enum PokeapiError {
    #[error("Missing '{0}' attribute")]
    MissingAttribute(String),
}

#[derive(Error, Debug)]
pub enum FuntranslationsError {
    #[error("{0}")]
    BadResponse(String),
}