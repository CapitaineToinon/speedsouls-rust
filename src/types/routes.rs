use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/leaderboards")]
    LeaderboardsRoot,
    #[at("/leaderboards/*")]
    Leaderboards,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum LeaderboardsRoute {
    #[at("/leaderboards")]
    Home,
    #[at("/leaderboards/:game_id")]
    Game { game_id: String },
    #[at("/leaderboards/:game_id/:category_id")]
    CategoryRoot { game_id: String, category_id: String },
    #[not_found]
    #[at("/leaderboards/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum CategoryRoute {
    #[at("/leaderboards/:id")]
    Home { id: String },
    #[at("/leaderboards/:game_id/:category_id")]
    Category {
        game_id: String,
        category_id: String,
    },
    #[not_found]
    #[at("/leaderboards/:game_id/404")]
    NotFound,
}
