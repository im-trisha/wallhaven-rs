mod query;

use bon::Builder;
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
#[builder(finish_fn(vis = "", name = build_internal))]
#[skip_serializing_none]
pub struct SearchRequest {
    /// A list of items of which the query is composed of
    ///
    /// See [`SearchQueryItem`] for more information
    #[serde_as(as = "Option<StringWithSeparator::<SpaceSeparator, SearchQueryItem>>")]
    #[serde(rename = "q")]
    query: Option<Vec<SearchQueryItem>>,
    /// Turn categories on or off
    categories: Option<Categories>,
    /// Turn categories on or off, NSFW requires a valid api key
    purity: Option<Purities>,
    /// The sorting type.
    sorting: Option<SortingType>,
    /// The sorting order
    order: Option<SortingOrder>,
    /// The toplist range.
    ///
    /// Sorting MUST be set to [`SortingType::Toplist`]
    #[serde(rename = "topRange")]
    toplist_range: Option<ToplistRange>,
    #[serde(rename = "atleast")]
    /// Minimum resolution allowed
    at_least: Option<Resolution>,
    /// List of exact wallpaper resolutions
    resolutions: Option<Vec<Resolution>>,
    /// List of aspect ratios
    ratios: Option<Vec<AspectRatio>>,
    /// Search by color
    #[serde(rename = "colors")]
    #[serde_as(as = "Option<DisplayFromStr>")]
    color: Option<Color>,
    /// Pagination, from 1->inf
    ///
    /// (Not actually infinite)
    page: Option<u64>,
    /// Optional seed for random results
    seed: Option<String>,
}

impl<S: search_request_builder::IsComplete> SearchRequestBuilder<S> {
    /// Builds the request.
    ///
    /// # Errors
    ///
    /// Throws an error if `toplist_range` is set and `sorting` isn't `SortingType::Toplist`
    pub fn build(self) -> Result<SearchRequest, String> {
        let req = self.build_internal();

        match (req.toplist_range, req.sorting) {
            (Some(_), Some(SortingType::Toplist)) => Ok(req),
            (Some(_), _) => Err("toplist_range is set and sorting != Toplist".to_string()),
            _ => Ok(req),
        }
    }
}
