//! Please refactor this to a `macro_rules!` Im too ass with them to do it myself, for now

mod categories;
mod purities;

pub use categories::Categories;
pub use purities::Purities;
use serde::Deserialize;

// Allow either a single string or a list of strings
#[derive(Deserialize)]
#[serde(untagged)]
enum OneOrManyFlaggables {
    One(String),
    Many(Vec<String>),
}
