use reqwest::get;

use crate::{
    error::Error,
    funtranslations::{funtranslations, Language},
    model::{Mode, Pokemon},
};

pub(crate) async fn pokemon(name: &str, mode: Mode) -> Result<Pokemon, Error> {
    let pokemon_resp = get(format!("https://pokeapi.co/api/v2/pokemon/{}", name))
        .await?
        .json::<serde_json::Value>()
        .await?;

    let species_url = pokemon_resp["species"].as_object().unwrap()["url"]
        .as_str()
        .unwrap();
    let species_resp = reqwest::get(species_url)
        .await?
        .json::<serde_json::Value>()
        .await?;
    let is_legendary = species_resp["is_legendary"].as_bool().unwrap();
    let habitat = species_resp["habitat"].as_object().unwrap()["name"]
        .as_str()
        .unwrap();
    let flavor_text = species_resp["flavor_text_entries"].as_array().unwrap();
    let mut description: Option<String> = None;
    for desc in flavor_text {
        if desc["language"].as_object().unwrap()["name"]
            .as_str()
            .unwrap()
            == "en"
        {
            description = Some(desc["flavor_text"].as_str().unwrap().to_owned());
        }
    }
    if let Mode::Translated = mode {
        if let Some(desc) = description.as_mut() {
            if let Some(trans) = funtranslations(desc, Language::Yoda).await? {
                description = Some(trans);
            }
        }
    }
    Ok(Pokemon::new(
        name,
        &description.unwrap(),
        habitat,
        is_legendary,
    ))
}
