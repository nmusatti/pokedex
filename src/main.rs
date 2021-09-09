mod api;
mod error;
mod funtranslations;
mod model;
mod pokeapi;

use rocket::routes;

use crate::api::{plain, translated};
use crate::error::Error;

#[rocket::main]
async fn main() -> Result<(), Error> {
    Ok(rocket::build()
        .mount("/pokemon", routes![plain, translated])
        .launch()
        .await?)
}
