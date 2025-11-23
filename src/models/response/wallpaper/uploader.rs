use serde::{Deserialize, Serialize};

use super::Avatar;
use crate::UserGroup;

/// The uploader of a wallpaper.
#[derive(Serialize, Deserialize)]
pub struct Uploader {
    /// Username of the uploader
    pub username: String,
    /// In which group the uploader belongs to
    pub group: UserGroup,
    /// The uploader's avatar
    pub avatar: Avatar,
}
