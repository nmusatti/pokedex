use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    PokeapiError(#[from] PokeapiError),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    RocketError(#[from] rocket::Error),
}

#[derive(Error, Debug)]
pub enum PokeapiError {
    #[error("Missing '{0}' attribute")]
    MissingAttribute(String),
}
