mod api;
mod backend;
mod error;
mod model;

use rocket::{routes, Config};

use crate::api::{plain, translated};
use crate::error::Error;

#[rocket::main]
async fn main() -> Result<(), Error> {
    let rocket = rocket::build().mount("/pokemon", routes![plain, translated]);

    let figment = rocket
        .figment()
        .clone()
        .merge((Config::PORT, 8000))
        .merge((Config::ADDRESS, "0.0.0.0"));

    Ok(rocket.configure(figment).launch().await?)
}
