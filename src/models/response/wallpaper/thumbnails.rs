use serde::{Deserialize, Serialize};

/// The thumbnail of a wallpaper, in different sizes
#[derive(Serialize, Deserialize)]
pub struct Thumbnails {
    /// Matches `ThumbnailSize::Large`
    pub large: String,
    /// Matches `ThumbnailSize::Original`
    pub original: String,
    /// Matches `ThumbnailSize::Small`
    pub small: String,
}
