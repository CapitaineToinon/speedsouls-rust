use crate::components::heroicons::mini::ArrowRight;
use crate::layouts::Default;
use crate::types::routes::LeaderboardsRoute;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Default>
            <div class="px-4 mx-auto max-w-8xl lg:px-4 lg:text-center py-12">
                <h1 class="mb-4 text-4xl font-bold tracking-tight text-gray-900 lg:font-extrabold lg:text-6xl lg:leading-none dark:text-white lg:text-center xl:px-36 lg:mb-7">
                    {"Going Fast in Boletaria, Lordran, Drangleic, Yharnam, Lothric and the Lands Between"}
                </h1>
                <p class="mb-10 text-lg font-normal text-gray-500 dark:text-gray-400 lg:text-center lg:text-xl xl:px-60">
                    {"SpeedSouls was established in September 2014 as a community hub for people seeking information about speedrunning the various FROMSoftware Souls games.
                    With the availability of Discord about a year later, we have formed a community for everyone interested. Nowadays, SpeedSouls is one of the largest groups focused on speedrunning a single game series."}
                </p>
                <Link<LeaderboardsRoute>
                    to={LeaderboardsRoute::Home}
                    classes="inline-flex justify-center gap-1 items-center py-3 px-5 text-base font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 dark:focus:ring-blue-900"
                >
                    {"Leaderboards"}
                    <ArrowRight />
                </Link<LeaderboardsRoute>>
            </div>
        </Default>
    }
}
