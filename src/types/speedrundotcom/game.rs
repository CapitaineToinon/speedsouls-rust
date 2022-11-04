use super::{Category, Response};
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Game {
    pub id: String,
    pub names: GameNames,
    pub assets: GameAssets,
    pub categories: Response<Vec<Category>>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct GameNames {
    pub international: String,
    pub twitch: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct GameAssets {
    pub background: Asset,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Asset {
    pub uri: String,
}
