mod query;
mod tag_query;
mod wallpaper_summary;

pub use query::*;
use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};
pub use tag_query::*;
pub use wallpaper_summary::*;

/// Search results, see properties for more information
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SearchResult {
    /// The actual found wallpapers
    pub data: Vec<WallpaperSummary>,
    /// Pagination/Query metadata
    pub meta: Meta,
}

/// Meta informations for the search results, used mostly to continue the search
#[serde_as]
#[derive(Deserialize, Clone, Debug)]
pub struct Meta {
    /// The current page
    pub current_page: u64,
    /// The last page (e.g. page count)
    pub last_page: u64,
    #[serde_as(as = "DisplayFromStr")]
    /// How many wallpapers per page
    pub per_page: u64,
    /// Total wallpapers count
    ///
    /// Meaning every page, except the last, will have [`total`] wallpapers, you can know how many there are in the last by doing:
    /// `(last_page * per_page) - total`
    pub total: u64,
    /// The query information, see [`Query`] variants for more informations
    pub query: Option<Query>,
    /// Random seed used when using [`SortingType::Random`]
    ///
    /// This can be fed back into the search request to not repeat wallpapers
    pub seed: Option<String>,
}
