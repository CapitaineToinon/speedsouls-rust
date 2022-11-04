use super::{use_local_storage, UseLocalStorageHandle};
use std::ops::Deref;
use std::rc::Rc;
use yew::prelude::*;

pub struct UseDarkHandle {
    color_scheme: UseLocalStorageHandle<String>,
    is_dark: Rc<bool>,
}

impl UseDarkHandle {
    pub fn set(&self, value: bool) {
        let mode = if value { "dark" } else { "light" };
        self.color_scheme.set(mode.to_string());
    }
}

impl Deref for UseDarkHandle {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &(*self.is_dark)
    }
}

impl Clone for UseDarkHandle {
    fn clone(&self) -> Self {
        Self {
            color_scheme: self.color_scheme.clone(),
            is_dark: self.is_dark.clone(),
        }
    }
}

impl PartialEq for UseDarkHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.color_scheme == *other.color_scheme
    }
}

#[hook]
pub fn use_dark() -> UseDarkHandle {
    let color_scheme = use_local_storage::<String>("yew-color-scheme".to_string());

    let is_dark = use_memo(
        |color_scheme| {
            let test = &**color_scheme;
            match &*test {
                Some(c) => *c == "dark".to_string(),
                None => false,
            }
        },
        color_scheme.clone(),
    );

    {
        let is_dark = is_dark.clone();

        use_effect_with_deps(
            |is_dark| {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();

                let class = match *is_dark {
                    true => "dark",
                    false => "light",
                };

                document.document_element().unwrap().set_class_name(class);
            },
            *is_dark,
        );
    }

    UseDarkHandle {
        color_scheme,
        is_dark,
    }
}
