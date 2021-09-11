mod api;
mod error;
mod funtranslations;
mod model;
mod pokeapi;

use rocket::{catchers, routes};

use crate::api::{not_found, plain, translated};
use crate::error::Error;

#[rocket::main]
async fn main() -> Result<(), Error> {
    Ok(rocket::build()
        .mount("/pokemon", routes![plain, translated])
        .register("/", catchers![not_found])
        .launch()
        .await?)
}
