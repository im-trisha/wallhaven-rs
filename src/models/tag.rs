use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub alias: String,
    pub category_id: i64,
    pub category: String,
    pub purity: String,
    pub created_at: String,
}
