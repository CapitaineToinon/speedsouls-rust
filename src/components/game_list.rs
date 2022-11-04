use crate::components::GameListItem;
use crate::types::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct GameListProps {
    pub games: Vec<speedrundotcom::Game>,
}

#[function_component(GameList)]
pub fn game_list(props: &GameListProps) -> Html {
    let GameListProps { games } = props;

    html! {
        <div class="grid grid-cols-3 gap-3">
            {games.iter().map(|g| html! {
                <GameListItem game={g.to_owned()} />
            }).collect::<Html>()}
        </div>
    }
}
