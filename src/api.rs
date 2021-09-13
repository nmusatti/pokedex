use rocket::{get, http::Status, response::status::Custom, serde::json::Json, Either};

use crate::{
    backend::backend,
    error::Error,
    model::{Mode, Pokemon},
};

async fn process(name: &str, mode: Mode) -> Either<Json<Pokemon>, Custom<String>> {
    let pokeapi = backend();
    match pokeapi.pokemon(name, mode).await {
        Ok(pokemon) => Either::Left(Json(pokemon)),
        Err(err) => match &err {
            Error::Http(http) => {
                Either::Right(Custom(Status::new(http.status), format!("{}", http)))
            }
            Error::NotFound(_) => Either::Right(Custom(Status::NotFound, err.to_string())),
            _ => Either::Right(Custom(Status::InternalServerError, format!("{}", err))),
        },
    }
}

#[get("/<name>")]
pub(crate) async fn plain(name: &str) -> Either<Json<Pokemon>, Custom<String>> {
    process(name, Mode::Plain).await
}

#[get("/translated/<name>")]
pub(crate) async fn translated(name: &str) -> Either<Json<Pokemon>, Custom<String>> {
    process(name, Mode::Translated).await
}
