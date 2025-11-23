use serde::{Deserialize, Serialize};
use serde_with::{BoolFromInt, serde_as};

/// An user's collection of wallpapers
///
/// User collections are just a list of wallpaper of some category
/// (categorized by the user that created it)
///
/// "Favorite wallpapers" is just a private collection, you can find it under the name "Default"
#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct UserCollection {
    /// The collection's id
    pub id: u64,
    /// The collection's name
    pub label: String,
    /// How many times this collection was seen
    pub views: u64,
    /// Whether this collection is private or not
    #[serde_as(as = "BoolFromInt")]
    pub public: bool,
    /// How many wallpapers are contained in this collection
    pub count: u64,
}
