use rocket::serde::Serialize;

#[derive(Serialize)]
pub(crate) struct Pokemon {
    name: String,
    description: String,
    habitat: String,
    is_legendary: bool,
}

impl Pokemon {
    pub(crate) fn new(name: &str, description: &str, habitat: &str, is_legendary: bool) -> Self {
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
            habitat: habitat.to_owned(),
            is_legendary
        }
    }
}

pub(crate) enum Mode {
    Plain,
    Translated
}