use reqwest::get;

use serde::Deserialize;

use crate::{error::{Error, HttpError}, model::{Language, Mode, Pokemon, Translator}};

#[derive(Deserialize, Debug)]
struct SpeciesRef {
    url: String,
}

#[derive(Deserialize, Debug)]
struct Individual {
    species: SpeciesRef,
}

#[derive(Deserialize, Debug)]
struct Habitat {
    name: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Lang {
    name: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Flavor {
    flavor_text: Option<String>,
    language: Option<Lang>,
}

#[derive(Deserialize, Debug)]
struct Species {
    #[serde(default)]
    is_legendary: bool,
    habitat: Option<Habitat>,
    flavor_text_entries: Vec<Flavor>,
}

pub(crate) struct Pokeapi {
    translator: Box<dyn Translator>
}

impl Pokeapi {
    pub(crate) fn new(translator: Box<dyn Translator>) -> Self {
        Self{ translator }
    }

    pub(crate) async fn pokemon(&self, name: &str, mode: Mode) -> Result<Pokemon, Error> {
        let individual_resp = get(format!("https://pokeapi.co/api/v2/pokemon/{}", name))
            .await;
        if let Err(err) = individual_resp {
            return Err(HttpError::extract(err));
        }
        let individual = individual_resp.unwrap().json::<Individual>().await?;
    
        let species_resp = get(individual.species.url).await;
        if let Err(err) = species_resp {
            return Err(HttpError::extract(err));
        }
        let species = species_resp.unwrap().json::<Species>().await?;
    
        let habitat = species.habitat.unwrap().name.unwrap();
        let mut description: Option<String> = None;
        for desc in species.flavor_text_entries {
            if desc.language.unwrap().name.unwrap() == "en" {
                description = Some(desc.flavor_text.unwrap());
            }
        }
        if let Mode::Translated = mode {
            let lang = if habitat == "cave" || species.is_legendary {
                Language::Yoda
            } else {
                Language::Shakespeare
            };
            if let Some(desc) = description.as_mut() {
                if let Ok(trans) = self.translator.translate(desc, lang).await {
                    description = Some(trans);
                }
            }
        }
        Ok(Pokemon::new(
            name,
            &description.unwrap(),
            &habitat,
            species.is_legendary,
        ))
    }
    
}
