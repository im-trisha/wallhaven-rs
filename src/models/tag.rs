use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct RawTagInfo {
    pub data: Tag,
}

#[derive(Serialize, Deserialize)]
pub struct Tag {
    pub id: u64,
    pub name: String,
    pub alias: String,
    pub category_id: u64,
    pub category: String,
    pub purity: String,
    pub created_at: String,
}
