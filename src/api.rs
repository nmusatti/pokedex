use rocket::{get, serde::json::Json};

use crate::{
    funtranslations::Funtranslations,
    model::{Mode, Pokemon},
    pokeapi::Pokeapi,
};

fn build() -> Pokeapi {
    Pokeapi::new(Box::new(Funtranslations::new()))
}

#[get("/<name>")]
pub(crate) async fn plain(name: &str) -> Json<Pokemon> {
    let pokeapi = build();
    Json(pokeapi.pokemon(name, Mode::Plain).await.unwrap())
}

#[get("/translated/<name>")]
pub(crate) async fn translated(name: &str) -> Json<Pokemon> {
    let pokeapi = build();
    Json(pokeapi.pokemon(name, Mode::Translated).await.unwrap())
}
