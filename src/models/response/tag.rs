use serde::{Deserialize, Serialize};

use crate::Purities;

/// A tag to represent a wallpaper, which is itself categorized into categories
#[derive(Serialize, Deserialize)]
pub struct Tag {
    /// This tag's id
    pub id: u64,
    /// The precise tag name
    pub name: String,
    /// An alias for this tag's `name`, used for fuzzy search
    pub alias: String,
    /// This tag's category's id
    ///
    /// Each tag is part of a category, for example, if two tags have the same category id, it means they are related somehow
    ///
    /// For example, the [weapon tag](https://wallhaven.cc/api/v1/tag/2980) has a `category_id` of 60 (which corresponds to a `category` = "Military & Weapons")
    ///
    /// In the same way, the [arrows tag](https://wallhaven.cc/api/v1/tag/2995) has the same `category_id` (thus, the same `category`)
    ///
    /// ## Note
    ///
    /// - There are a lot of tag categories, like, A LOT. Its not feasible to add every one of them to an enum, unfortunately.
    /// - To my knowledge, there is no way to fetch tags from the api. I am asking the developers though! TODO
    pub category_id: u64,
    /// A string representation of the `category_id`, in english
    ///
    /// See `category_id` for more informations
    pub category: String,
    /// The general purity of the tag
    ///
    /// As far as I'm aware of, on tags this purity can either be sfw or nsfw, no sketchy.
    /// Do not trust this information, and if you prove me wrong please fill an issue or make a PR!
    /// (Be sure to put an example wallpaper id for me to test)
    ///
    /// Even if the type is `Purities`, dont be fooled:
    ///
    /// It actually can only be one purity!
    pub purity: Purities,
    /// The date of creation of this tag
    pub created_at: jiff::civil::DateTime,
}
