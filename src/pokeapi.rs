use reqwest::get;

use serde::Deserialize;

use crate::{
    error::Error,
    funtranslations::{funtranslations, Language},
    model::{Mode, Pokemon},
};

#[derive(Deserialize, Debug)]
struct SpeciesRef {
    url: Option<String>
}

#[derive(Deserialize, Debug)]
struct Individual {
    species: Option<SpeciesRef>
}

#[derive(Deserialize, Debug)]
struct Habitat {
    name: Option<String>
}

#[derive(Deserialize, Debug)]
struct Lang {
    name: Option<String>
}

#[derive(Deserialize, Debug)]
struct Flavor {
    flavor_text: Option<String>,
    language: Option<Lang>
}

#[derive(Deserialize, Debug)]
struct Species {
    is_legendary: Option<bool>,
    habitat: Option<Habitat>,
    flavor_text_entries: Vec<Flavor>
}

pub(crate) async fn pokemon(name: &str, mode: Mode) -> Result<Pokemon, Error> {
    let individual = get(format!("https://pokeapi.co/api/v2/pokemon/{}", name))
        .await?
        .json::<Individual>()
        .await?;

    let species_url = individual.species.unwrap().url.unwrap();
    let species = get(species_url)
        .await?
        .json::<Species>()
        .await?;
    let is_legendary = species.is_legendary.unwrap();
    let habitat = species.habitat.unwrap().name.unwrap();
    let flavor_text = species.flavor_text_entries;
    let mut description: Option<String> = None;
    for desc in flavor_text {
        if desc.language.unwrap().name.unwrap() == "en" {
            description = Some(desc.flavor_text.unwrap());
        }
    }
    if let Mode::Translated = mode {
        let lang = if habitat == "cave" || is_legendary {
            Language::Yoda
        } else {
            Language::Shakespeare
        };
        if let Some(desc) = description.as_mut() {
            if let Ok(trans) = funtranslations(desc, lang).await {
                description = Some(trans);
            }
        }
    }
    Ok(Pokemon::new(
        name,
        &description.unwrap(),
        &habitat,
        is_legendary,
    ))
}
