mod api;
mod error;
mod funtranslations;
mod model;
mod pokeapi;

use rocket::{catchers, routes, Config};

use crate::api::{not_found, plain, translated};
use crate::error::Error;

#[rocket::main]
async fn main() -> Result<(), Error> {
    let rocket = rocket::build()
        .mount("/pokemon", routes![plain, translated])
        .register("/", catchers![not_found]);

    let figment = rocket
        .figment()
        .clone()
        .merge((Config::PORT, 8000))
        .merge((Config::ADDRESS, "0.0.0.0"));

    Ok(rocket.configure(figment).launch().await?)
}
