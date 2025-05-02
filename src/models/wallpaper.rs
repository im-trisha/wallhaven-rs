use serde::{Deserialize, Serialize};

use super::{tag::Tag, Thumbs};

#[derive(Serialize, Deserialize)]
pub(crate) struct RawWallpaperInfo {
    pub data: Wallpaper,
}

#[derive(Serialize, Deserialize)]
pub struct Wallpaper {
    pub id: String,
    pub url: String,
    pub short_url: String,
    pub uploader: Uploader,
    pub views: i64,
    pub favorites: i64,
    pub source: String,
    pub purity: String,
    pub category: String,
    pub dimension_x: i64,
    pub dimension_y: i64,
    pub resolution: String,
    pub ratio: String,
    pub file_size: i64,
    pub file_type: String,
    pub created_at: String,
    pub colors: Vec<String>,
    pub path: String,
    pub thumbs: Thumbs,
    pub tags: Vec<Tag>,
}


#[derive(Serialize, Deserialize)]
pub struct Uploader {
    pub username: String,
    pub group: String,
    pub avatar: Avatar,
}

#[derive(Serialize, Deserialize)]
pub struct Avatar {
    #[serde(rename = "200px")]
    pub px200: String,
    #[serde(rename = "128px")]
    pub px128: String,
    #[serde(rename = "32px")]
    pub px32: String,
    #[serde(rename = "20px")]
    pub px20: String,
}
