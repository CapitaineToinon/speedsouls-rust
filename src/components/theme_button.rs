use super::heroicons::mini::{Moon, Sun};
use crate::hooks::use_dark;
use yew::prelude::*;

#[function_component(ThemeButton)]
pub fn theme_button() -> Html {
    let dark = use_dark();

    let onclick = {
        let dark = dark.clone();
        Callback::from(move |_| dark.set(!*dark))
    };

    html! {
        <button {onclick} class="text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-700 rounded-lg text-sm p-2.5">
            {match *dark {
                true => html! { <Sun /> },
                false => html! { <Moon /> },
            }}
        </button>
    }
}
