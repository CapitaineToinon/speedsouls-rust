use crate::components::theme_button::ThemeButton;
use crate::types::routes::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Properties)]
struct NavbarLinkProps {
    to: Route,
    label: String,
    active: bool,
}

#[function_component(NavbarLink)]
fn navbar_link(NavbarLinkProps { to, label, active }: &NavbarLinkProps) -> HtmlResult {
    Ok(html! {
        <li>
            <Link<Route>
                to={to.to_owned()}
                classes={classes!(
                    "block",
                    "py-2",
                    "pr-4",
                    "pl-3",
                    "rounded",
                    active.then(|| Some("md:bg-transparent md:text-blue-700 md:p-0 dark:text-white")),
                    (!active).then(|| Some("hover:bg-gray-100 md:hover:bg-transparent md:hover:text-blue-700 md:p-0 md:dark:hover:text-white dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent dark:border-gray-700"))
                )}
            >{label}</Link<Route>>
        </li>
    })
}

#[function_component(Navbar)]
pub fn navbar() -> HtmlResult {
    let route: Route = use_route().unwrap();
    let collapsed = use_state_eq(|| true);

    let onclick = {
        let c = collapsed.clone();
        move |_| {
            c.set(!*c);
        }
    };

    let routes: [(Route, String); 2] = [
        (Route::Home, "Home".into()),
        (Route::LeaderboardsRoot, "Leaderboards".into()),
    ];

    Ok(html! {
        <nav class="bg-white px-2 sm:px-4 border-b dark:bg-gray-800 border-gray-200 dark:border-gray-600">
            <div class="container flex flex-wrap justify-between items-center mx-auto">
                <Link<Route>
                    to={Route::Home}
                    classes={"flex items-center"}
                >
                    <img
                        src="https://flowbite.com/docs/images/logo.svg"
                        class="mr-3 h-6 sm:h-9" alt="Flowbite Logo"
                    />
                    <span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white">
                        {"SpeedSouls"}
                    </span>
                </Link<Route>>
                <button
                    {onclick}
                    data-collapse-toggle="navbar-default"
                    type="button"
                    class="inline-flex items-center p-2 ml-3 text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600"
                    aria-controls="navbar-default"
                    aria-expanded="false"
                >
                    <span class="sr-only">
                        {"Open main menu"}
                    </span>
                    <svg
                        class="w-6 h-6"
                        aria-hidden="true"
                        fill="currentColor"
                        viewBox="0 0 20 20"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                            fill-rule="evenodd"
                            d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z"
                            clip-rule="evenodd"
                        ></path>
                    </svg>
                </button>
                <div class={classes!(
                    "w-full",
                    "md:block",
                    "md:w-auto",
                    collapsed.then(|| Some("hidden")),
                )}>
                    <ul class="flex flex-col items-center p-4 mt-4 bg-gray-50 rounded-lg border border-gray-100 md:flex-row md:space-x-8 md:mt-0 md:text-sm md:font-medium md:border-0 md:bg-white dark:bg-gray-800 dark:border-gray-700">
                        {routes.iter().map(|(to, label)| {
                            html ! {
                                <NavbarLink
                                    to={to.to_owned()}
                                    label={label.to_owned()}
                                    active={to == &route}
                                />
                            }
                        }).collect::<Html>()}
                        <li>
                            <ThemeButton />
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
    })
}
