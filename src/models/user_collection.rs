use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct RawUserCollection {
    pub data: Vec<UserCollection>,
}

#[derive(Serialize, Deserialize)]
pub struct UserCollection {
    pub id: i64,
    pub label: String,
    pub views: i64,
    pub public: i64,
    pub count: i64,
}
