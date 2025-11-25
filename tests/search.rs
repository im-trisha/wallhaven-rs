//! Test the 2 client creation functions, `new` and `with_key`

use assert2::let_assert;
use common::ensure_api_key;
use wallhaven_rs::{Categories, Purities, Query, SearchQueryItem, SearchRequest, WallhavenClient};
mod common;

#[tokio::test]
async fn test_search_simple() {
    let key = ensure_api_key();
    let_assert!(Ok(client) = WallhavenClient::with_key(key));

    let_assert!(Ok(res) = client.search(None).await);
    assert!(!res.data.is_empty());
}

#[tokio::test]
async fn test_search_query() {
    let key = ensure_api_key();
    let_assert!(Ok(client) = WallhavenClient::with_key(key));

    let test_query = vec![
        SearchQueryItem::Exact("boobs".to_string()), // Dont judge
        SearchQueryItem::Exact("thighs".to_string()), // Dont judge
        SearchQueryItem::Exclude("weapons".to_string()),
    ];

    let_assert!(
        Ok(query) = SearchRequest::builder()
            .categories(Categories::ANIME)
            // Can't be searched "by accident", so if the results have NSFW, the query worked
            .purity(Purities::NSFW)
            .query(test_query)
            .build(),
    );

    let_assert!(Ok(res) = client.search(Some(query)).await);
    assert!(!res.data.is_empty());

    let first = res.data.first().unwrap();

    let_assert!(Some(Query::Text(meta_query)) = res.meta.query);
    assert!(meta_query == "+boobs +thighs -weapons");
    assert!(first.purity.contains(Purities::NSFW));
    assert!(first.category.contains(Categories::ANIME));
}
