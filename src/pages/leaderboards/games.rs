use yew::prelude::*;
use yew::suspense::use_future;

use crate::api::speedrundotcom::fetch_games;
use crate::components::GameList;
use crate::layouts::Default;

#[function_component(Content)]
fn content() -> HtmlResult {
    let result = use_future(|| async { fetch_games().await })?;

    Ok(match *result {
        Ok(ref games) => html! { <GameList games={games.to_owned()} /> },
        Err(ref failure) => failure.to_string().into(),
    })
}

#[function_component(Games)]
pub fn game() -> HtmlResult {
    let fallback = html! {<div>{"Loading..."}</div>};

    Ok(html! {
        <Default>
            <Suspense {fallback}>
                <Content />
            </Suspense>
        </Default>
    })
}
