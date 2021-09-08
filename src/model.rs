use rocket::{
    serde::Serialize,
};

#[derive(Serialize)]
pub(crate) struct Pokemon {
    name: String,
}

impl Pokemon {
    pub(crate) fn new(name: &str) -> Self {
        Self { name: name.to_owned() }
    }
}
