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
        match get(format!("https://pokeapi.co/api/v2/pokemon/{}", name)).await {
            Err(err) => Err(HttpError::extract(err)),
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(resp.json::<Individual>().await?)
                } else if resp.status().as_u16() == 404 {
                    Err(Error::NotFound("Individual".to_owned()))
                } else {
                    Err(HttpError::from_message(
                        resp.status().as_u16(),
                        "Error retrieving individual",
                    )
                    .into())
                }
            }
        }
    }

    async fn get_species(url: &str) -> Result<Species, Error> {
        match get(url).await {
            Err(err) => Err(HttpError::extract(err)),
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(resp.json::<Species>().await?)
                } else if resp.status().as_u16() == 404 {
                    Err(Error::NotFound("Species".to_owned()))
                } else {
                    Err(
                        HttpError::from_message(resp.status().as_u16(), "Error retrieving species")
                            .into(),
                    )
                }
            }
        }
    }

    pub(crate) async fn pokemon(&self, name: &str, mode: Mode) -> Result<Pokemon, Error> {
        let individual = Self::get_individual(name).await?;
        let species = Self::get_species(&individual.species.url).await?;
        let flavor = species
            .flavor_text_entries
            .iter()
            .find(|f| f.language.name == "en");
        if flavor.is_none() {
            return Err(Error::NotFound("Description".to_owned()));
        }
        let mut description = flavor.unwrap().flavor_text.to_owned();
        if let Mode::Translated = mode {
            let lang = if species.habitat.name == "cave" || species.is_legendary {
                Language::Yoda
            } else {
                Language::Shakespeare
            };
            if let Ok(trans) = self.translator.translate(&description, lang).await {
                description = trans;
            }
        }
        Ok(Pokemon::new(
            name,
            &description,
            &species.habitat.name,
            species.is_legendary,
        ))
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use crate::{
        error::Error,
        funtranslations::Funtranslations,
        model::{Mode, Translator},
    };

    use super::Pokeapi;

    struct DummyTranslator {}

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

    #[tokio::test]
    async fn translated_pokemon_ok() -> Result<(), Error> {
        let pokeapi = Pokeapi::new(Box::new(Funtranslations::new()));
        let pokemon = pokeapi.pokemon("regice", Mode::Translated).await?;
        assert_eq!(pokemon.habitat, "cave");
        Ok(())
    }
}
