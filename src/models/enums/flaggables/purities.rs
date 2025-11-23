use bitflags::bitflags;
use serde::{Deserialize, Serialize, de};

use super::OneOrManyFlaggables;

bitflags! {
    /// Purities (the level of lewdiness) to filter by
    ///
    /// Check the variants for more information
    #[derive(Clone, Copy, Debug)]
    pub struct Purities: u8 {
        /// NOT safe for work, may contain nudity.
        ///
        ///  I still use some of them when I feel like it, still weird looks from coworkers
        const NSFW = 1 << 0;
        /// Sketchy, meaning neither safe nor unsafe for work, depends on your entourage
        ///
        /// Coworkers/friends may call you strange, but just do whatever makes you happy! I use those on all my devices :)
        const SKETCHY = 1 << 1;
        /// Safe for work! There may be anime girls in it, but who cares?
        const SFW = 1 << 1;
    }
}

impl Purities {
    const SFW_NAME: &'static str = "sfw";
    const SKETCHY_NAME: &'static str = "sketchy";
    const NSFW_NAME: &'static str = "nsfw";

    const POSSIBLE_NAMES: &[&'static str] = &[Self::SFW_NAME, Self::SKETCHY_NAME, Self::NSFW_NAME];
}

impl Serialize for Purities {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{self:03b}"))
    }
}

impl<'de> Deserialize<'de> for Purities {
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
                Self::SFW_NAME => Ok(Self::SFW),
                Self::SKETCHY_NAME => Ok(Self::SKETCHY),
                Self::NSFW_NAME => Ok(Self::NSFW),
                variant => Err(de::Error::unknown_variant(variant, Self::POSSIBLE_NAMES)),
            })
            .try_fold(Self::empty(), |a, b| Ok(a | b?))
    }
}
