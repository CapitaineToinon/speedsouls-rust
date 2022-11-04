use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::{hooks::use_navigator, Routable};

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct ReplaceProps<R: Routable> {
    pub to: R,
}

#[function_component(Replace)]
pub fn replace<R>(props: &ReplaceProps<R>) -> Html
where
    R: Routable + 'static,
{
    let history = use_navigator().expect_throw("failed to read history.");
    let target_route = props.to.clone();

    use_effect(move || {
        history.replace(&target_route);
    });

    Html::default()
}
