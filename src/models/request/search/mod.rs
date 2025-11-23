mod query;

use derive_builder::Builder;
pub use query::*;
use serde::Serialize;
use serde_with::{
    DisplayFromStr, StringWithSeparator, formats::SpaceSeparator, serde_as, skip_serializing_none,
};

use crate::{
    AspectRatio, Categories, Color, Purities, Resolution, SortingOrder, SortingType, ToplistRange,
};

/// The search request object.
#[serde_as]
#[derive(Serialize, Builder, Clone, Debug)]
#[builder(build_fn(validate = "Self::validate"))]
#[skip_serializing_none]
pub struct SearchRequest {
    /// A list of items of which the query is composed of
    ///
    /// See [`SearchQueryItem`] for more information
    #[serde_as(as = "Option<StringWithSeparator::<SpaceSeparator, SearchQueryItem>>")]
    #[serde(rename = "q")]
    #[builder(setter(into, strip_option), default)]
    query: Option<Vec<SearchQueryItem>>,
    /// Turn categories on or off
    #[builder(setter(into, strip_option), default)]
    categories: Option<Categories>,
    /// Turn categories on or off, NSFW requires a valid api key
    #[builder(setter(into, strip_option), default)]
    purity: Option<Purities>,
    /// The sorting type.
    #[builder(setter(into, strip_option), default)]
    sorting: Option<SortingType>,
    /// The sorting order
    #[builder(setter(into, strip_option), default)]
    order: Option<SortingOrder>,
    /// The toplist range.
    ///
    /// Sorting MUST be set to [`SortingType::Toplist`]
    #[serde(rename = "topRange")]
    #[builder(setter(into, strip_option), default)]
    toplist_range: Option<ToplistRange>,
    #[serde(rename = "atleast")]
    /// Minimum resolution allowed
    #[builder(setter(into, strip_option), default)]
    at_least: Option<Resolution>,
    /// List of exact wallpaper resolutions
    #[builder(setter(into, strip_option), default)]
    resolutions: Option<Vec<Resolution>>,
    /// List of aspect ratios
    #[builder(setter(into, strip_option), default)]
    ratios: Option<Vec<AspectRatio>>,
    /// Search by color
    #[serde(rename = "colors")]
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[builder(setter(into, strip_option), default)]
    color: Option<Color>,
    /// Pagination, from 1->inf
    ///
    /// (Not actually infinite)
    #[builder(setter(into, strip_option), default)]
    page: Option<u64>,
    /// Optional seed for random results
    #[builder(setter(into, strip_option), default)]
    seed: Option<String>,
}

impl SearchRequestBuilder {
    fn validate(&self) -> Result<(), String> {
        match (self.toplist_range.flatten(), self.sorting.flatten()) {
            (Some(_), Some(SortingType::Toplist)) => Ok(()),
            (Some(_), _) => Err("toplist_range is set and sorting != Toplist".to_string()),
            _ => Ok(()),
        }
    }
}
