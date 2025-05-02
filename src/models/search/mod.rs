mod request;

use serde::{Deserialize, Serialize};

pub use request::*;

#[derive(Serialize, Deserialize)]
pub struct SearchResult {
    pub data: Vec<super::Wallpaper>,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub current_page: i64,
    pub last_page: i64,
    pub per_page: i64,
    pub total: i64,
    pub query: Option<Query>,
    pub seed: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Query {
    Text(String),
    Object(TagQuery),
}

#[derive(Serialize, Deserialize)]
pub struct TagQuery {
    pub id: i64,
    pub tag: String,
}
