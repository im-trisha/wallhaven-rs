use serde::Deserialize;

use super::*;

#[derive(Deserialize)]
pub struct RawUserSettings {
    pub data: UserSettings,
}

#[derive(Deserialize)]
pub struct RawUserCollection {
    pub data: Vec<UserCollection>,
}

#[derive(Deserialize)]
pub struct RawTagInfo {
    pub data: Tag,
}
#[derive(Deserialize)]
pub struct RawWallpaperDetails {
    pub data: WallpaperDetails,
}
