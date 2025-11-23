use serde::Deserialize;

/// The thumbnail of a wallpaper, in different sizes
#[derive(Deserialize, Clone, Debug)]
pub struct Thumbnails {
    /// Matches `ThumbnailSize::Large`
    pub large: String,
    /// Matches `ThumbnailSize::Original`
    pub original: String,
    /// Matches `ThumbnailSize::Small`
    pub small: String,
}
