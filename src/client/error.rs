use std::fmt::Display;

use reqwest::header::InvalidHeaderValue;

#[derive(Debug)]
pub enum Error {
    InvalidApiKey,
    ErrorParsingPath,
    IoError(std::io::Error),
    UrlParsingError(url::ParseError),
    BuildingClient(reqwest::Error),
    SendingRequest(reqwest::Error),
    DecodingJson(reqwest::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidApiKey => write!(f, "Invalid api key!"),
            Self::ErrorParsingPath => write!(
                f,
                "While parsing this wallpaper's url, we couldn't detect the extension."
            ),
            Self::IoError(err) => write!(f, "{}", err),
            Self::UrlParsingError(err) => write!(f, "{}", err),
            Self::BuildingClient(err) => write!(f, "{}", err),
            Self::SendingRequest(err) => write!(f, "{}", err),
            Self::DecodingJson(err) => write!(f, "{}", err),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        match value.is_decode() {
            true => Error::DecodingJson(value),
            false => match value.is_request() {
                true => Error::SendingRequest(value),
                false => Error::BuildingClient(value),
            },
        }
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(_: InvalidHeaderValue) -> Self {
        Error::InvalidApiKey
    }
}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Error::UrlParsingError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}
