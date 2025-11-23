//! Test for `wallpaper` and `download_wallpaper` function function

use assert2::let_assert;
use common::ensure_api_key;
use futures::StreamExt;
use wallhaven_rs::WallhavenClient;
mod common;

#[tokio::test]
async fn test_wallpaper_and_download() {
    let key = ensure_api_key();
    let_assert!(Ok(client) = WallhavenClient::with_key(key));

    let_assert!(Ok(res) = client.wallpaper("lyy23p").await);
    let_assert!(Ok(mut stream) = client.download_wallpaper(&res).await);

    assert!(stream.next().await.is_some());

    let_assert!(Ok(mut thumbnail_stream) = client.download_thumbnail(&res.thumbnails, wallhaven_rs::ThumbnailResolution::Small).await);

    assert!(thumbnail_stream.next().await.is_some());
}
