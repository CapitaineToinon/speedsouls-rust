use crate::types::routes::*;
use crate::types::speedrundotcom::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
pub struct GameListItemProps {
    pub game: Game,
}

#[function_component(GameListItem)]
pub fn game_list_item(props: &GameListItemProps) -> Html {
    let GameListItemProps { game } = props;

    html! {
        <Link<LeaderboardsRoute>
            to={LeaderboardsRoute::Game {
                game_id: game.id.to_string()
            }}
            classes="relative cursor-pointer filter shadow-lg"
        >
            <div>
                <img
                    class="rounded-lg aspect-video"
                    src={game.assets.background.uri.to_string()}
                    alt={game.names.international.to_string()}
                />
            </div>
            <div class="absolute inset-0 flex items-center justify-center rounded-lg overflow-hidden bg-black bg-opacity-50 text-lg text-white font-bold">
                <p>{game.names.international.to_string()}</p>
            </div>
        </Link<LeaderboardsRoute>>
    }
}
