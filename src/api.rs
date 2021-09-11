use rocket::{Either, Request, catch, get, http::Status, response::status::Custom, serde::json::Json};

use crate::{error::Error, funtranslations::Funtranslations, model::{Mode, Pokemon}, pokeapi::Pokeapi};


async fn process(name: &str, mode: Mode) -> Either<Json<Pokemon>, Custom<String>> {
    let pokeapi = Pokeapi::new(Box::new(Funtranslations::new()));
    match pokeapi.pokemon(name, mode).await {
        Ok(pokemon) => Either::Left(Json(pokemon)),
        Err(err) => {
            match &err {
                Error::Http(http) => Either::Right(Custom(Status::new(http.status), format!("{}", http))),
                Error::NotFound(_) => Either::Right(Custom(Status::NotFound, err.to_string())),
                _ => Either::Right(Custom(Status::InternalServerError, format!("{}", err)))
            }
        }
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

#[catch(404)]
pub(crate) fn not_found(req: &Request) -> String {
    format!("'{}' not found", req.uri())
}