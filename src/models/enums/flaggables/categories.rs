use bitflags::bitflags;
use serde::{Deserialize, Serialize, de};

use super::OneOrManyFlaggables;

bitflags! {
    /// Categories to filter the wallpapers by
    ///
    /// Check the variants for more information
    #[derive(Clone, Copy, Debug)]
    pub struct Categories: u8 {
        /// People. You know, the thing you (hopefully) see every day
        const PEOPLE = 1 << 0;
        /// Anime/Manga, you may want to look that up on your favorite search engine if you dont know what that is
        const ANIME = 1 << 1;
        /// Anything that isn't anime/manga or people, for example, a picture of nature
        const GENERAL = 1 << 2;
    }
}

impl Categories {
    const GENERAL_NAME: &'static str = "general";
    const ANIME_NAME: &'static str = "anime";
    const PEOPLE_NAME: &'static str = "people";

    const POSSIBLE_NAMES: &[&'static str] =
        &[Self::GENERAL_NAME, Self::ANIME_NAME, Self::PEOPLE_NAME];
}

impl Serialize for Categories {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{self:03b}"))
    }
}

impl<'de> Deserialize<'de> for Categories {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let input = OneOrManyFlaggables::deserialize(deserializer)?;
        let items: Vec<String> = match input {
            OneOrManyFlaggables::One(s) => vec![s],
            OneOrManyFlaggables::Many(v) => v,
        };

        items
            .into_iter()
            .map(|s| match s.as_str() {
                Self::GENERAL_NAME => Ok(Self::GENERAL),
                Self::ANIME_NAME => Ok(Self::ANIME),
                Self::PEOPLE_NAME => Ok(Self::PEOPLE),
                variant => Err(de::Error::unknown_variant(variant, Self::POSSIBLE_NAMES)),
            })
            .try_fold(Self::empty(), |a, b| Ok(a | b?))
    }
}
