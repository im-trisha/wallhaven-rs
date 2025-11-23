use serde::{Deserialize, Serialize};

/// The property to sort by.
///
/// Check the variants for more information.
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SortingType {
    /// Sort by date added
    DateAdded,
    /// Sort by how much the wallpaper respects the filters
    Relevance,
    /// Sort randomly
    Random,
    /// Sort by views count
    Views,
    /// Sort by ⭐ count
    Favorites,
    /// Sort by popularity within a certain time span
    Toplist,
}
