use std::{fmt::Display, str::FromStr};

use crate::FileType;

/// A single search query item
#[derive(Clone, Debug)]
pub enum SearchQueryItem {
    /// Fuzzily search for a tag/keyword
    ///
    /// On the web UI, this corresponds to `tag`
    Fuzz(String),
    /// Exclude a tag/keyword
    ///
    /// On the web UI, this corresponds to `-tag`
    Exclude(String),
    /// Exact tag match, without the fuzzy search
    ///
    /// On the web UI, this corresponds to `+tag`
    Exact(String),
    /// Exact uploader match
    ///
    /// On the web UI, this corresponds to `@username`
    FromUser(String),
    /// Exact tag id search (cannot be combined)
    ///
    /// On the web UI, this corresponds to `id:123`
    TagById(u64),
    /// The wallpaper file type
    ///
    /// On the web UI, this corresponds to `type:{jpg/png}`
    FileType(FileType),
    /// Find wallpapers with similar tags
    ///
    /// On the web UI, this corresponds to `like:{wallpaper_id}`
    LikeWallpaper(String),
}

impl Display for SearchQueryItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fuzz(s) => write!(f, "{s}",),
            Self::Exclude(s) => write!(f, "-{s}"),
            Self::Exact(s) => write!(f, "+{s}"),
            Self::FromUser(u) => write!(f, "@{u}"),
            Self::TagById(id) => write!(f, "id:{id}"),
            Self::FileType(s) => write!(f, "type:{s}"),
            Self::LikeWallpaper(s) => write!(f, "like:{s}"),
        }
    }
}

impl FromStr for SearchQueryItem {
    type Err = std::convert::Infallible;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        unreachable!()
    }
}
