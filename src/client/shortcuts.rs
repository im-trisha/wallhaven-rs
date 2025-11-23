use futures::Stream;

use super::{Error, WallhavenClient};
use crate::WallpaperDetails;

impl WallpaperDetails {
    /// Shorthand for [`WallhavenClient::download_wallpaper`]
    ///
    /// # Errors
    ///
    /// See [`WallhavenClient::download_wallpaper`]
    pub async fn download(
        &self,
        client: &WallhavenClient,
    ) -> Result<impl Stream<Item = reqwest::Result<bytes::Bytes>>, Error> {
        client.download_wallpaper(self).await
    }
}
