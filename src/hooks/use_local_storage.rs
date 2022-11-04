use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::rc::Rc;
use yew::prelude::*;

pub struct UseLocalStorageHandle<T> {
    inner: UseStateHandle<Option<T>>,
    key: Rc<String>,
}

impl<T> UseLocalStorageHandle<T> {
    pub fn set(&self, value: T)
    where
        T: Serialize + Clone,
    {
        if LocalStorage::set(&*self.key, value.clone()).is_ok() {
            self.inner.set(Some(value));
        }
    }
}

impl<T> Deref for UseLocalStorageHandle<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &(*self.inner)
    }
}

impl<T> Clone for UseLocalStorageHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            key: self.key.clone(),
        }
    }
}

impl<T> PartialEq for UseLocalStorageHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

#[hook]
pub fn use_local_storage<T>(key: String) -> UseLocalStorageHandle<T>
where
    T: for<'de> Deserialize<'de> + 'static,
{
    let inner: UseStateHandle<Option<T>> =
        use_state(|| LocalStorage::get(&key).unwrap_or_default());
    let key = use_memo(|_| key, ());

    UseLocalStorageHandle { inner, key }
}
