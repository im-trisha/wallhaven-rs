use std::num::ParseIntError;

use thiserror::Error;

/// Possible errors inside the `wallhaven-rs` crate
#[derive(Error, Debug)]
pub enum Error {
    /// Returned when an invalid api key is provided to `WallhavenClient::with_key`
    #[error("Invalid api key.")]
    InvalidApiKey(#[from] reqwest::header::InvalidHeaderValue),
    /// Returned when a request fails to build the url from the provided parameters,
    /// usually means the parameters are wrong
    #[error(transparent)]
    UrlParsingError(#[from] url::ParseError),
    /// Returned when building `WallhavenClient`
    ///
    /// This error is thrown when building the `rewqest` client,
    /// this means that its probably the library's fault if this happens (it probably will not happen though)
    /// and you should open an issue!
    #[error("Error building the client: {0}")]
    BuildingClient(reqwest::Error),
    /// There was some error while sending the request, you should match the underlying [`reqwest::Error`] further
    #[error("Error sending a request: {0}")]
    SendingRequest(reqwest::Error),
    /// There was an error decoding the JSON, but the response was received successfully.
    ///
    /// Its an error either on wallhaven's side, if they sent a wrong json, or our side, if we wrote a bad model
    #[error("Error deconding the request json: {0}")]
    DecodingJson(reqwest::Error),
    /// Some request error that isn't neither [`Error::SendingRequest`] nor [`Error::DecodingJson`]
    ///
    /// You can match further the underlying [`reqwest::Error`]
    #[error("Unknown request error: {0}")]
    UnknownRequestError(reqwest::Error),
}

/// Used only by `models::enums::color::Color` to parse `FromStr`
#[derive(Error, Debug)]
pub enum ColorParseError {
    /// An invalid hex value was given
    #[error("Invalid hex value. {}", .0)]
    InvalidHex(#[from] ParseIntError),
    /// An invalid color was given, not in the enum range
    #[error("Invalid color value.")]
    InvalidValue,
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        if value.is_builder() {
            Self::BuildingClient(value)
        } else if value.is_decode() {
            Self::DecodingJson(value)
        } else if value.is_request() {
            Self::SendingRequest(value)
        } else {
            Self::UnknownRequestError(value)
        }
    }
}
