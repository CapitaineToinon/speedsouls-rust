use crate::api::speedrundotcom::fetch_game;
use crate::components::Replace;
use crate::types::routes::CategoryRoute;
use yew::prelude::*;
use yew::suspense::use_future_with_deps;
use yew_router::prelude::Redirect;

#[derive(PartialEq, Properties)]
pub struct CategoryRedirectProps {
    pub id: String,
}

#[function_component(Content)]
fn content(props: &CategoryRedirectProps) -> HtmlResult {
    let result = use_future_with_deps(
        |id| async move { fetch_game(id.to_string()).await },
        props.id.to_string(),
    )?;

    Ok(match *result {
        Ok(ref game) => {
            let category = game.categories.data.first();

            match category {
                Some(ref category) => html! {
                    <Replace<CategoryRoute>
                        to={CategoryRoute::Category {
                            game_id: game.id.to_owned(),
                            category_id: category.id.to_owned(),
                        }}
                    />
                },
                None => html! {
                    <Redirect<CategoryRoute> to={CategoryRoute::NotFound} />
                },
            }
        }
        Err(ref failure) => failure.to_string().into(),
    })
}

#[function_component(CategoryRedirect)]
pub fn game(props: &CategoryRedirectProps) -> HtmlResult {
    let CategoryRedirectProps { id } = props;

    let fallback = html! {<div>{"Loading..."}</div>};

    Ok(html! {
        <div class="container mx-auto py-3">
            <Suspense {fallback}>
                <Content id={id.to_owned()} />
            </Suspense>
        </div>
    })
}
