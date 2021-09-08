use serde_json::Value;

use crate::{
    error::{Error, PokeapiError},
    model::Pokemon,
};

pub(crate) async fn pokemon(name: &str) -> Result<Pokemon, Error> {
    let resp = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}", name))
        .await?
        .json::<serde_json::Value>()
        .await?;

    match &resp["name"] {
        Value::String(name) => Ok(Pokemon::new(name)),
        _ => Err(PokeapiError::MissingAttribute("name".to_owned()).into()),
    }
}
