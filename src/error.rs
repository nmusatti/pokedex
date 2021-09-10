use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Http(#[from] HttpError),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Rocket(#[from] rocket::Error),
}

#[derive(Error, Debug)]
pub struct HttpError {
    status: u16,
    msg: String,
    source: reqwest::Error
}

impl HttpError {
    pub(crate) fn new(status: u16, msg: String, source: reqwest::Error) -> Self {
        Self{ status, msg, source }
    }

    pub(crate) fn extract(err: reqwest::Error) -> Error {
        if let Some(status) = err.status() {
            HttpError::new(status.as_u16(), "Translation error".to_owned(), err).into()
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
