use crate::types::routes::*;
use crate::types::speedrundotcom::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CategoryListItemProps {
    pub game: Game,
    pub category: Category,
    pub active: bool,
    pub first: bool,
    pub last: bool,
}

#[function_component(CategoryListItem)]
pub fn game_list_item(props: &CategoryListItemProps) -> Html {
    let CategoryListItemProps {
        game,
        category,
        active,
        first,
        last,
    } = props;

    html! {
        <Link<CategoryRoute>
            to={CategoryRoute::Category {
                game_id: game.id.to_string(),
                category_id: category.id.to_string()
            }}
            classes={classes!(
                "block",
                "py-2",
                "px-4",
                "w-full",
                first.then(|| "rounded-t-lg"),
                last.then(|| "rounded-b-lg"),
                (!last).then(|| "border-b border-gray-200 dark:border-gray-600"),
                active.then(|| "text-white bg-blue-700 dark:bg-gray-800"),
                (!active).then(|| "focus:outline-none focus:ring-2 focus:ring-blue-700 focus:text-blue-700 hover:bg-gray-100 hover:text-blue-700 dark:hover:bg-gray-600 dark:hover:text-white dark:focus:ring-gray-500 dark:focus:text-white"),
            )}
        >
            {&category.name}
        </Link<CategoryRoute>>
    }
}
