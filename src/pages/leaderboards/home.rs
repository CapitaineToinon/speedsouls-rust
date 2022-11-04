use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::leaderboards::game::Game;
use crate::pages::leaderboards::games::Games;
use crate::types::routes::*;

fn switch(routes: LeaderboardsRoute) -> Html {
    match routes {
        LeaderboardsRoute::Home => html! { <Games />  },
        LeaderboardsRoute::Game { game_id } => html! { <Game {game_id} /> },
        LeaderboardsRoute::CategoryRoot {
            game_id,
            category_id,
        } => html! {
        <Game
            {game_id}
            {category_id}
        /> },
        LeaderboardsRoute::NotFound => html! { <Redirect<Route> to={Route::NotFound}/> },
    }
}

#[function_component(LeaderboardsHome)]
pub fn home() -> Html {
    html! {
        <Switch<LeaderboardsRoute> render={switch} />
    }
}
