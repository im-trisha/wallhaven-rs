use serde::Deserialize;
use serde_with::{DefaultOnError, DisplayFromStr, serde_as};

use crate::{AspectRatio, Categories, Purities, Resolution, ThumbnailResolution, ToplistRange};

/// Represents user settings, mostly the default search settings
#[serde_as]
#[derive(Deserialize)]
pub struct UserSettings {
    /// The default wallpaper thumbnail resolution
    pub thumb_size: ThumbnailResolution,
    /// How many wallpapers per page by default
    #[serde_as(as = "DisplayFromStr")]
    pub per_page: u64,
    /// Purities used by default when searching.
    ///
    /// See [`Purities`]
    pub purity: Purities,
    /// Categories used by default when searching.
    ///
    /// See [`Categories`]
    pub categories: Categories,
    /// A list of resolutions that will be included by default when searching.
    ///
    /// See [`Resolution`]
    #[serde_as(as = "DefaultOnError<_>")]
    pub resolutions: Vec<Resolution>,
    /// A list of aspect ratios that will be included by default when searching.
    ///
    /// See [`AspectRatio`]
    #[serde_as(as = "DefaultOnError<_>")]
    pub aspect_ratios: Vec<AspectRatio>,
    /// The toplist range used by default when searching.
    ///
    /// See [`ToplistRange`]
    pub toplist_range: ToplistRange,
    /// A list of tags that are excluded from all searches and listings
    pub tag_blacklist: Vec<String>,
    /// A list of usernames that are excluded from all searches and listings
    pub user_blacklist: Vec<String>,
}
