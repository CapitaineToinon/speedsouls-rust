mod api;
mod components;
mod hooks;
mod layouts;
mod pages;
mod types;
use pages::leaderboards::home::LeaderboardsHome;
use pages::Home;
use types::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::LeaderboardsRoot | Route::Leaderboards => html! { <LeaderboardsHome /> },
        Route::NotFound => html! { <div>{"the location you've gone to doesn't exist :("}</div>},
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
