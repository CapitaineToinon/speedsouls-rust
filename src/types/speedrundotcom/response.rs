use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Response<T> {
    pub data: T,
}