//! Test for `tag` function

use assert2::let_assert;
use common::ensure_api_key;
use wallhaven_rs::WallhavenClient;
mod common;

#[tokio::test]
async fn test_tag() {
    let key = ensure_api_key();
    let_assert!(Ok(client) = WallhavenClient::with_key(key));

    let_assert!(Ok(res) = client.tag(186).await);

    assert!(res.name == "ponytail");
}
