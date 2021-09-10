use reqwest::get;

use serde::Deserialize;

use crate::{
    error::{Error, HttpError},
    model::{Language, Mode, Pokemon, Translator},
};

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
    name: String,
}

#[derive(Deserialize, Debug)]
struct Lang {
    name: String,
}

#[derive(Deserialize, Debug)]
struct Flavor {
    flavor_text: String,
    language: Lang,
}

#[derive(Deserialize, Debug)]
struct Species {
    #[serde(default)]
    is_legendary: bool,
    habitat: Habitat,
    flavor_text_entries: Vec<Flavor>,
}

pub(crate) struct Pokeapi {
    translator: Box<dyn Translator>,
}

impl Pokeapi {
    pub(crate) fn new(translator: Box<dyn Translator>) -> Self {
        Self { translator }
    }

    async fn get_individual(name: &str) -> Result<Individual, Error> {
        let individual_resp = get(format!("https://pokeapi.co/api/v2/pokemon/{}", name)).await;
        if let Err(err) = individual_resp {
            return Err(HttpError::extract(err));
        }
        Ok(individual_resp.unwrap().json::<Individual>().await?)
    }

    async fn get_species(url: &str) -> Result<Species, Error> {
        let species_resp = get(url).await;
        if let Err(err) = species_resp {
            return Err(HttpError::extract(err));
        }
        Ok(species_resp.unwrap().json::<Species>().await?)
    }

    pub(crate) async fn pokemon(&self, name: &str, mode: Mode) -> Result<Pokemon, Error> {
        let individual = Self::get_individual(name).await?;
        let species = Self::get_species(&individual.species.url).await?;

        let mut description: Option<String> = None;
        for desc in species.flavor_text_entries {
            if desc.language.name == "en" {
                description = Some(desc.flavor_text);
                break;
            }
        }
        if let None = description {
            return Err(Error::MissingData("description".to_owned()));
        }
        if let Mode::Translated = mode {
            let lang = if species.habitat.name == "cave" || species.is_legendary {
                Language::Yoda
            } else {
                Language::Shakespeare
            };
            if let Ok(trans) = self.translator.translate(&description.as_ref().unwrap(), lang).await {
                description = Some(trans);
            }
        }
        Ok(Pokemon::new(name, &description.unwrap(), &species.habitat.name, species.is_legendary))
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use crate::{error::Error, model::{Mode, Translator}};

    use super::Pokeapi;

    struct DummyTranslator {

    }

    #[async_trait]
    impl Translator for DummyTranslator {
        async fn translate(&self, text: &str, _: crate::model::Language) -> Result<String, Error> {
            Ok(text.to_owned())
        }
    }
    
    #[tokio::test]
    async fn plain_pokemon_ok() -> Result<(), Error> {
        let pokeapi = Pokeapi::new(Box::new(DummyTranslator {}));
        let pokemon = pokeapi.pokemon("butterfree", Mode::Plain).await?;
        assert_eq!(pokemon.habitat, "forest");
        Ok(())
    }
}
