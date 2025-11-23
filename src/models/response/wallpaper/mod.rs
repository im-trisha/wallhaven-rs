mod avatar;
mod thumbnails;
mod uploader;

pub use avatar::Avatar;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
pub use thumbnails::Thumbnails;
pub use uploader::Uploader;

use crate::{Categories, Color, Purities, Tag};

/// The main character on wallhaven x)
///
/// This struct represents a wallpaper in wallhaven's database
#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct WallpaperDetails {
    /// The wallpaper's id on wallhaven's database
    pub id: String,
    /// The wallpaper's url, will match `https://wallhaven.cc/w/{id}`
    #[serde_as(as = "DisplayFromStr")]
    pub url: url::Url,
    /// The wallpaper's short url, will match `https://whvn.cc/{id}`
    #[serde_as(as = "DisplayFromStr")]
    pub short_url: url::Url,
    /// The wallpaper's uploader
    pub uploader: Uploader,
    /// How many times this wallpaper was seen
    pub views: u64,
    /// How many times this wallpaper was starred
    pub favorites: u64,
    /// The wallpaper's source
    ///
    /// Most of the time will be `""` or an url, but it may be anything
    pub source: String,
    /// The purity of the wallpaper
    ///
    /// Even if the type is `Purities`, dont be fooled:
    ///
    /// It actually can only be one purity!
    pub purity: Purities,
    /// The category of the wallpaper
    ///
    /// Even if the type is `Categories`, dont be fooled:
    ///
    /// It actually can only be one category!
    pub category: Categories,
    #[serde(rename = "dimension_x")]
    /// The x dimension (width)
    pub width: u64,
    #[serde(rename = "dimension_y")]
    /// The y dimension (height)
    pub height: u64,
    /// The resolution
    ///
    /// This is a string instead of `Resolution` because this can be any resolution,
    /// thus not expressible by the `Resolution` enum. Solutions are welcome
    pub resolution: String,
    /// The wallpaper's ratio
    ///
    /// Should be the same as `(width/height).ceil()`
    #[serde_as(as = "DisplayFromStr")]
    pub ratio: f32,
    /// The wallpaper's file size in bytes
    pub file_size: u64,
    /// The wallpaper's mime type
    ///
    /// You can derive the file extension from this
    #[serde_as(as = "DisplayFromStr")]
    pub file_type: mime::Mime,
    /// The wallpaper's creation datetime
    pub created_at: jiff::civil::DateTime,
    /// A list of colors that match this wallpaper
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub colors: Vec<Color>,
    /// The wallpaper's download url
    #[serde_as(as = "DisplayFromStr")]
    pub path: url::Url,
    /// The wallpaper's thumbnails
    pub thumbs: Thumbnails,
    /// The wallpaper's tags
    pub tags: Vec<Tag>,
}
