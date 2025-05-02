use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct RawUserSettings {
    pub data: UserSettings,
}

#[derive(Serialize, Deserialize)]
pub struct UserSettings {
    pub thumb_size: String,
    pub per_page: String,
    pub purity: Vec<String>,
    pub categories: Vec<String>,
    pub resolutions: Vec<String>,
    pub aspect_ratios: Vec<String>,
    pub toplist_range: String,
    pub tag_blacklist: Vec<String>,
    pub user_blacklist: Vec<String>,
}
