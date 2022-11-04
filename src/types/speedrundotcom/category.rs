use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
}
