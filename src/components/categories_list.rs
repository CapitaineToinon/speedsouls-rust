use super::CategoryListItem;
use crate::types::speedrundotcom::{Category, Game};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CategoryListProps {
    pub game: Game,
    pub categories: Vec<Category>,
    #[prop_or_default]
    pub active_category_id: String,
}

#[function_component(CategoryList)]
pub fn categories_list(props: &CategoryListProps) -> HtmlResult {
    let CategoryListProps {
        game,
        categories,
        active_category_id,
    } = props;

    Ok(html! {
        <div class="w-64 text-sm font-medium text-gray-900 bg-white rounded-lg border border-gray-200 dark:bg-gray-700 dark:border-gray-600 dark:text-white">
            {categories.iter().enumerate().map(|(index, category)| html! {
                <CategoryListItem
                    game={game.to_owned()}
                    category={category.to_owned()}
                    active={&category.id == active_category_id}
                    first={index == 0}
                    last={index == categories.len() - 1}
                />
            }).collect::<Html>()}
        </div>
    })
}
