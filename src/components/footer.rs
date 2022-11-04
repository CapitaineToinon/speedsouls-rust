use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="p-4 border-t border-gray-200 dark:border-gray-600 bg-white shadow md:flex md:items-center md:justify-between md:p-6 dark:bg-gray-800">
            <div class="container mx-auto text-center">
                <span class="text-sm text-gray-500 sm:text-center dark:text-gray-400">
                    {"© 2022 "}<a href="https://flowbite.com/" class="hover:underline">{"Flowbite™"}</a>{". All Rights Reserved."}
                </span>
            </div>
        </footer>
    }
}
