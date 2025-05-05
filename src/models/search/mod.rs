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
    pub current_page: u64,
    pub last_page: u64,
    pub per_page: u64,
    pub total: u64,
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
    pub id: u64,
    pub tag: String,
}
