use serde::{Deserialize};

/// A struct representing a tag query
#[derive(Deserialize, Clone, Debug)]
pub struct TagQuery {
    /// The id of the tag (can be used to look up more informations about it)
    pub id: u64,
    /// The tag name (e.g. [`Tag::name`])
    pub tag: String,
}
