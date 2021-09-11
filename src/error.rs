use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum Error {
    #[error(transparent)]
    Http(#[from] HttpError),
    #[error("{0} not found")]
    NotFound(String),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Rocket(#[from] rocket::Error),
}

#[derive(Error, Debug)]
pub(crate) struct HttpError {
    pub(crate) status: u16,
    msg: String,
    source: Option<reqwest::Error>,
}

impl HttpError {
    pub(crate) fn from_error(status: u16, source: reqwest::Error) -> Self {
        Self {
            status,
            msg : source.to_string(),
            source: Some(source),
        }
    }

    pub(crate) fn from_message(status: u16, msg: &str) -> Self {
        Self {
            status,
            msg: msg.to_owned(),
            source: None
        }
    }
    pub(crate) fn extract(err: reqwest::Error) -> Error {
        if let Some(status) = err.status() {
            HttpError::from_error(status.as_u16(), err).into()
        } else {
            err.into()
        }
    }
}

impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Status: {} - {}", self.status, self.msg)
    }
}
