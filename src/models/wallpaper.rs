use serde::{Deserialize, Serialize};

use super::{Thumbs, tag::Tag};

#[derive(Serialize, Deserialize)]
pub(crate) struct RawWallpaperInfo {
    pub data: Wallpaper,
}

#[derive(Serialize, Deserialize)]
pub struct Wallpaper {
    pub id: String,
    pub url: String,
    pub short_url: String,
    pub uploader: Option<Uploader>,
    pub views: u64,
    pub favorites: u64,
    pub source: String,
    pub purity: String,
    pub category: String,
    pub dimension_x: u64,
    pub dimension_y: u64,
    pub resolution: String,
    pub ratio: String,
    pub file_size: u64,
    pub file_type: String,
    pub created_at: String,
    pub colors: Vec<String>,
    pub path: String,
    pub thumbs: Thumbs,
    pub tags: Option<Vec<Tag>>,
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
