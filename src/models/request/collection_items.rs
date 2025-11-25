use bon::Builder;
use serde::Serialize;
use serde_with::{serde_as, skip_serializing_none};

use crate::Purities;

/// The collection items request object.
///
/// Must be used with its builder [`CollectionItemsRequestBuilder`]
#[serde_as]
#[derive(Serialize, Builder, Clone, Debug)]
#[skip_serializing_none]
pub struct CollectionItemsRequest {
    /// Turn categories on or off, NSFW requires a valid api key
    purity: Option<Purities>,
    /// Pagination, from 1->inf
    ///
    /// (Not actually infinite)
    page: Option<u64>,
}
