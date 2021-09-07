use rocket::{
    get, routes,
    serde::{json::Json, Serialize},
    Error,
};

#[derive(Serialize)]
struct Pokemon {
    name: String,
}

async fn pokemon(name: &str) -> Result<Pokemon,reqwest::Error> {
    let resp = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}", name))
    .await?
    .json::<serde_json::Value>()
    .await?;

    Ok(Pokemon{ name: resp["name"].to_string() })
}

#[get("/<name>")]
async fn plain(name: &str) -> Json<Pokemon> {
    Json(pokemon(name).await.unwrap())
}

#[get("/translated/<name>")]
async fn translated(name: &str) -> Json<Pokemon> {
    Json(pokemon(name).await.unwrap())
}

#[rocket::main]
async fn main() -> Result<(), Error> {
    Ok(rocket::build()
        .mount("/pokemon", routes![plain, translated])
        .launch()
        .await?)
}
