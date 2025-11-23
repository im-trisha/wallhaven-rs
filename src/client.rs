use std::{str::FromStr, sync::LazyLock};

use futures::Stream;
use reqwest::{Client, Url, header};

mod error;
mod has_path;

pub use error::*;
use has_path::*;

use crate::models::{raw_models::*, *};

/// A wallhaven api client.
///
/// There are two ways of constructing this object, with or without an api key,
/// see [`WallhavenClient::new`] and [`WallhavenClient::with_key`] for more details
///
/// ## Examples
///
/// ```rust,ignore
/// client.wallpaper("someid").await?;
/// client.search(None).await?;
///
/// let req = SearchRequestBuilder::default().build()?;
/// client.search(Some(req)).await?;
/// ```
///
/// And so on! Check out each methods/models for more informations
#[derive(Clone)]
pub struct WallhavenClient {
    client: Client,
}

static BASE_URL: LazyLock<Url> =
    LazyLock::new(|| Url::from_str("https://wallhaven.cc/api/v1/").unwrap());

macro_rules! request_errors {
        () => {
            "
# Errors

- [`Error::UrlParsingError`] when the URL cannot be parsed
- [`Error::SendingRequest`] an error while sending the request, you can get more details with the underlying error
- [`Error::DecodingJson`] an error decoding the JSON, either wallhaven sent a wrong json, or we wrote a bad model
- [`Error::UnknownRequestError`] - an unknown error when sending/receiving the request, you can match further the underlying error"
        };
    }

macro_rules! download_errors {
        () => {
            "
# Errors

- [`Error::SendingRequest`] an error while sending the request, you can get more details with the underlying error
- [`Error::UnknownRequestError`] - an unknown error when sending/receiving the request, you can match further the underlying error"
    };
}

impl WallhavenClient {
    /// Constructs [`WallhavenClient`] with an api key
    /// Check out [`WallhavenClient::new`] to construct an instance without an api key.
    /// The api key will be passed to the X-API-Key headers.
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidApiKey`] when an invalid api key is passed as argument
    /// - [`Error::BuildingClient`] when something went wrong with the Client builder, this shouldn't happen and you probably should file an issue
    pub fn with_key(api_key: impl AsRef<str>) -> Result<Self, Error> {
        let mut auth_header = header::HeaderValue::from_str(api_key.as_ref())?;
        auth_header.set_sensitive(true);

        let mut headers = header::HeaderMap::with_capacity(1);
        headers.insert("X-API-Key", auth_header);

        let client = Client::builder().default_headers(headers).build()?;
        Ok(Self { client })
    }

    /// Constructs [`WallhavenClient`] without an api key.
    /// Check out [`WallhavenClient::with_key`] to construct an instance with an api key.
    ///
    /// # Errors
    ///
    /// - [`Error::BuildingClient`] when something went wrong with the Client builder, this shouldn't happen and you probably should file an issue
    pub fn new() -> Result<Self, Error> {
        let client = Client::builder().build()?;
        Ok(Self { client })
    }

    /// Fetches a wallpaper by id
    #[doc = request_errors!()]
    pub async fn wallpaper(&self, id: impl AsRef<str>) -> Result<WallpaperDetails, Error> {
        let url = BASE_URL.join(&format!("w/{}", id.as_ref()))?;

        let res = self.client.get(url).send().await?;
        let raw_json: RawWallpaperDetails = res.json().await?;

        Ok(raw_json.data)
    }

    /// Searches for wallpapers matching the request
    #[doc = request_errors!()]
    pub async fn search(&self, params: Option<SearchRequest>) -> Result<SearchResult, Error> {
        let url = BASE_URL.join("search")?;

        // Build the request builder first
        let res = self.client.get(url).query(&params).send().await?;

        Ok(res.json().await?)
    }

    /// Fetches all the [`UserCollection`] of a certain user
    ///
    /// Defaults to the current if no username is provided and an api key is used
    #[doc = request_errors!()]
    pub async fn collections(
        &self,
        username: impl Into<Option<String>>,
    ) -> Result<Vec<UserCollection>, Error> {
        let url = username.into().map_or_else(
            || "collections".to_string(),
            |username| format!("collections/{username}"),
        );

        let url = BASE_URL.join(&url)?;

        let res = self.client.get(url).send().await?;
        let raw_json: RawUserCollection = res.json().await?;

        Ok(raw_json.data)
    }

    /// Gets the collection's wallpapers.
    #[doc = request_errors!()]
    pub async fn collection_items(
        &self,
        username: impl AsRef<str>,
        id: u64,
        params: Option<CollectionItemsRequest>,
    ) -> Result<SearchResult, Error> {
        let url = BASE_URL.join(&format!("collections/{}/{id}", username.as_ref()))?;

        let res = self.client.get(url).query(&params).send().await?;

        Ok(res.json().await?)
    }

    /// Fetches a [`Tag`] by its id
    #[doc = request_errors!()]
    pub async fn tag(&self, id: u64) -> Result<Tag, Error> {
        let url = BASE_URL.join(&format!("tag/{id}"))?;

        let res = self.client.get(url).send().await?;
        let raw_json: RawTagInfo = res.json().await?;

        Ok(raw_json.data)
    }

    /// Fetches the [`UserSettings`] of the current user. Only works if the api key was provided.
    #[doc = request_errors!()]
    pub async fn user_settings(&self) -> Result<UserSettings, Error> {
        let url = BASE_URL.join("settings")?;

        let res = self.client.get(url).send().await?;
        let raw_json: RawUserSettings = res.json().await?;

        Ok(raw_json.data)
    }

    /// Downloads a any wallpaper type (e.g. `WallhavenDetails` or `WallhavenSummary`)
    #[doc = download_errors!()]
    pub async fn download_wallpaper(
        &self,
        wallpaper: &impl HasPath,
    ) -> Result<impl Stream<Item = reqwest::Result<bytes::Bytes>>, Error> {
        let resp = self.client.get(wallpaper.path()).send().await?;
        Ok(resp.bytes_stream())
    }

    /// Downloads a [`Thumbnails`]
    #[doc = download_errors!()]
    pub async fn download_thumbnail(
        &self,
        thumbnail: &Thumbnails,
        resolution: ThumbnailResolution,
    ) -> Result<impl Stream<Item = reqwest::Result<bytes::Bytes>>, Error> {
        let url = match resolution {
            ThumbnailResolution::Large => thumbnail.large.clone(),
            ThumbnailResolution::Original => thumbnail.original.clone(),
            ThumbnailResolution::Small => thumbnail.small.clone(),
        };

        let resp = self.client.get(url).send().await?;
        Ok(resp.bytes_stream())
    }
}
