use rocket::{get, serde::json::Json};

use crate::{model::Pokemon, pokeapi::pokemon};


#[get("/<name>")]
pub(crate) async fn plain(name: &str) -> Json<Pokemon> {
    Json(pokemon(name).await.unwrap())
}

#[get("/translated/<name>")]
pub(crate) async fn translated(name: &str) -> Json<Pokemon> {
    Json(pokemon(name).await.unwrap())
}
