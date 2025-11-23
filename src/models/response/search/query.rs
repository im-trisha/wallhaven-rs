use serde::Deserialize;

use crate::TagQuery;

/// The tag query information
#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Query {
    /// The whole query as a string
    Text(String),
    /// For exact tag searches
    Object(TagQuery),
}
