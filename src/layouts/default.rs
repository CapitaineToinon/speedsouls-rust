use crate::components::{Footer, Navbar};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Default)]
pub fn default(props: &Props) -> Html {
    html! {
        <main class="min-h-screen bg-white dark:bg-gray-900 flex flex-col">
            <Navbar />
            <div class="container mx-auto p-3 flex-grow">
                { for props.children.iter() }
            </div>
            <Footer />
        </main>
    }
}
