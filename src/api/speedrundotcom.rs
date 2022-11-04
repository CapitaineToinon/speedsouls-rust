use crate::types::*;
use gloo_net::http::{Request, RequestCache, Response};
use gloo_net::Error;

async fn fetch(path: &str) -> Result<Response, Error> {
    let url = format!("https://www.speedrun.com/api/v1{}", path);
    Request::get(url.as_str())
        .cache(RequestCache::ForceCache)
        .send()
        .await
}

async fn fetch_json<T: for<'de> serde::Deserialize<'de>>(path: &str) -> Result<T, Error> {
    let resp = fetch(path).await?;
    resp.json::<T>().await
}

pub async fn fetch_games() -> Result<Vec<speedrundotcom::Game>, Error> {
    let resp = fetch_json::<speedrundotcom::Response<Vec<speedrundotcom::Game>>>(
        "/series/souls/games?embed=categories",
    )
    .await?;
    Ok(resp.data)
}

pub async fn fetch_game(id: String) -> Result<speedrundotcom::Game, String> {
    let games = fetch_games().await.unwrap();
    let game = games.iter().find(|&game| game.id == id);

    match game {
        Some(g) => Ok(g.to_owned()),
        None => Err("game not found".to_string()),
    }
}

pub async fn fetch_leaderboard(
    game_id: String,
    category_id: String,
) -> Result<speedrundotcom::Leaderboard, Error> {
    let path = format!("/leaderboards/{}/category/{}", game_id, category_id);
    let resp =
        fetch_json::<speedrundotcom::Response<speedrundotcom::Leaderboard>>(path.as_str()).await?;
    Ok(resp.data)
}
