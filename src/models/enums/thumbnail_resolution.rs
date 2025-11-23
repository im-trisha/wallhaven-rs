use serde::{Deserialize, Serialize};

/// The wallpaper's thumbnail resolution
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ThumbnailResolution {
    /// The best thumbnail resolution
    Large,
    #[serde(rename = "orig")]
    /// The "original" thumbnail resolution, not clear what that means
    Original,
    /// The lowest thumbnail resolution
    Small,
}
