use yew::prelude::*;
use yew::suspense::use_future_with_deps;

use crate::api::speedrundotcom::fetch_leaderboard;

#[derive(PartialEq, Properties)]
pub struct CategoryProps {
    pub game_id: String,
    pub category_id: String,
}

#[function_component(Content)]
fn content(props: &CategoryProps) -> HtmlResult {
    let result = use_future_with_deps(
        |deps| async move { fetch_leaderboard(deps.0.to_string(), deps.1.to_string()).await },
        (props.game_id.to_string(), props.category_id.to_string()),
    )?;

    Ok(match *result {
        Ok(ref leaderboard) => html! {
            <div class="flex gap-1">
                <div class="overflow-x-auto rounded-lg w-full relative border border-gray-200 dark:border-gray-600">
                    <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                        <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                            <tr>
                                <th scope="col" class="py-2 px-6">
                                    {"Place"}
                                </th>
                                <th scope="col" class="py-2 px-6">
                                    {"Link"}
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            {leaderboard.runs.iter().enumerate().map(|(index, run)| html!{
                                <tr
                                    classes={classes!(
                                        "bg-white",
                                        "dark:bg-gray-800",
                                        "dark:border-gray-700",
                                        (index != 0).then(|| "border-b")
                                    )}
                                >
                                    <th scope="row" class="py-2 px-6 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                                        {&run.place}
                                    </th>
                                    <td class="py-2 px-6">
                                        {&run.run.weblink}
                                    </td>
                                </tr>
                            }).collect::<Html>()}
                        </tbody>
                    </table>
                </div>
            </div>
        },
        Err(ref failure) => failure.to_string().into(),
    })
}

#[function_component(Category)]
pub fn game(props: &CategoryProps) -> HtmlResult {
    let CategoryProps {
        game_id,
        category_id,
    } = props;

    let fallback = html! {<div>{"Loading..."}</div>};

    Ok(html! {
        <div class="w-full">
            <Suspense {fallback}>
                <Content
                    game_id={game_id.to_owned()}
                    category_id={category_id.to_owned()}
                />
            </Suspense>
        </div>
    })
}
