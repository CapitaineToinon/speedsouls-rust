use yew::prelude::*;
use yew::suspense::use_future_with_deps;
use yew_router::prelude::Redirect;
use yew_router::Switch;

use crate::api::speedrundotcom::fetch_game;
use crate::components::CategoryList;
use crate::layouts::Default;
use crate::pages::category::Category;
use crate::types::routes::{CategoryRoute, Route};

use super::category_redirect::CategoryRedirect;

fn switch(routes: CategoryRoute) -> Html {
    match routes {
        CategoryRoute::Home { id } => html! {
            <CategoryRedirect
                {id}
            />
        },
        CategoryRoute::Category {
            game_id,
            category_id,
        } => html! {
           <Category
               {game_id}
               {category_id}
           />
        },
        CategoryRoute::NotFound => html! { <Redirect<Route> to={Route::NotFound}/> },
    }
}

#[derive(PartialEq, Properties)]
pub struct GameProps {
    pub game_id: String,
    #[prop_or_default]
    pub category_id: String,
}

#[function_component(Content)]
fn content(props: &GameProps) -> HtmlResult {
    let result = use_future_with_deps(
        |game_id| async move { fetch_game(game_id.to_string()).await },
        props.game_id.to_string(),
    )?;

    Ok(match *result {
        Ok(ref game) => html! {
            <div class="flex gap-3 items-start">
                <div class="sticky top-3">
                    <CategoryList
                        game={game.to_owned()}
                        categories={game.categories.data.to_owned()}
                        active_category_id={props.category_id.to_owned()}
                    />
                </div>
                <Switch<CategoryRoute> render={switch} />
            </div>
        },
        Err(ref failure) => failure.to_string().into(),
    })
}

#[function_component(Game)]
pub fn game(props: &GameProps) -> HtmlResult {
    let GameProps {
        game_id,
        category_id,
    } = props;

    let fallback = html! {<div>{"Loading..."}</div>};

    Ok(html! {
        <Default>
            <Suspense {fallback}>
                <Content
                    game_id={game_id.to_owned()}
                    category_id={category_id.to_owned()}
                />
            </Suspense>
        </Default>
    })
}
