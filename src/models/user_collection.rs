use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct RawUserCollection {
    pub data: Vec<UserCollection>,
}

#[derive(Serialize, Deserialize)]
pub struct UserCollection {
    pub id: u64,
    pub label: String,
    pub views: u64,
    pub public: u64,
    pub count: u64,
}
