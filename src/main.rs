use rocket::{get, routes, serde::{Serialize, json::Json}};


#[derive(Serialize)]
struct Pokemon<'p> {
    name: &'p str,
}

#[get("/<name>")]
fn plain(name: &str) -> Json<Pokemon> {
    Json(Pokemon{ name })
}

#[get("/translated/<name>")]
fn translated(name: &str) -> Json<Pokemon> {
    Json(Pokemon{ name })
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    Ok(rocket::build().mount("/pokemon", routes![plain, translated]).launch().await?)
}
