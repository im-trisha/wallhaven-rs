//! Test for `collections` function

use assert2::let_assert;
use common::ensure_api_key;
use wallhaven_rs::WallhavenClient;
mod common;

#[tokio::test]
async fn test_collections() {
    let key = ensure_api_key();
    let_assert!(Ok(client) = WallhavenClient::with_key(key));

    let_assert!(Ok(res) = client.collections(None).await);

    // This will indeed fail if the user renames their favorites collection
    // But I dont think anyone other than me will run those tests and I will not rename the collection
    assert!(!res.is_empty() && res.iter().any(|c| c.label == "Default"));
}
