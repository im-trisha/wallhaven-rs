use serde::Deserialize;

/// The thumbnail of a wallpaper, in different sizes
#[derive(Deserialize, Clone, Debug)]
pub struct Thumbnails {
    /// Matches `ThumbnailSize::Large`
    pub large: url::Url,
    /// Matches `ThumbnailSize::Original`
    pub original: url::Url,
    /// Matches `ThumbnailSize::Small`
    pub small: url::Url,
}
