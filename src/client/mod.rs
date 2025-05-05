use std::{path::Path, str::FromStr, sync::LazyLock};

use futures::StreamExt;
use reqwest::{Client, Url, header};
use tokio::fs::File;

mod error;
mod shortcuts;
mod utils;

pub use error::*;

use crate::models::*;

#[derive(Clone)]
pub struct WallhavenClient {
    client: Client,
}

static BASE_URL: LazyLock<Url> =
    LazyLock::new(|| Url::from_str("https://wallhaven.cc/api/v1/").unwrap());

impl WallhavenClient {
    pub fn new(api_key: Option<&str>) -> Result<Self, Error> {
        let builder = Client::builder();

        let builder = match api_key {
            None => builder,
            Some(key) => {
                let mut headers = header::HeaderMap::with_capacity(1);
                headers.insert("X-API-Key", header::HeaderValue::from_str(key)?);
                builder.default_headers(headers)
            }
        };

        let client = builder.build().map_err(|err| Error::from(err))?;
        Ok(Self { client })
    }

    pub async fn wallpaper(&self, id: &str) -> Result<Wallpaper, Error> {
        let url = BASE_URL.join(&format!("w/{id}"))?;

        let res = self.client.get(url).send().await?;
        let raw_json: RawWallpaperInfo = res.json().await?;

        Ok(raw_json.data)
    }

    async fn download(&self, url: &str, outpath: impl AsRef<Path>) -> Result<(), Error> {
        let resp = self.client.get(url).send().await?;
        let mut out = File::create(&outpath).await?;

        let mut bytes_stream = resp.bytes_stream();

        while let Some(item) = bytes_stream.next().await {
            tokio::io::copy(&mut item?.as_ref(), &mut out).await?;
        }

        Ok(())
    }

    pub async fn collections(&self, username: Option<&str>) -> Result<Vec<UserCollection>, Error> {
        let url = match username {
            None => "collections",
            Some(username) => &format!("collections/{username}"),
        };
        let url = BASE_URL.join(url)?;

        let res = self.client.get(url).send().await?;
        let raw_json: RawUserCollection = res.json().await?;

        Ok(raw_json.data)
    }

    pub async fn collection_items(&self, username: &str, id: u64) -> Result<SearchResult, Error> {
        let url = BASE_URL.join(&format!("collections/{username}/{id}"))?;

        let res = self.client.get(url).send().await?;

        Ok(res.json().await?)
    }

    pub async fn tag(&self, id: u64) -> Result<Tag, Error> {
        let url = BASE_URL.join(&format!("tag/{id}"))?;

        let res = self.client.get(url).send().await?;
        let raw_json: RawTagInfo = res.json().await?;

        Ok(raw_json.data)
    }

    pub async fn user_settings(&self) -> Result<UserSettings, Error> {
        let url = BASE_URL.join(&format!("settings"))?;

        let res = self.client.get(url).send().await?;
        let raw_json: RawUserSettings = res.json().await?;

        Ok(raw_json.data)
    }
}
