use std::fmt;
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, BungieApiError>;

#[derive(Debug)]
pub enum BungieApiError {
    InvalidContentType(reqwest::header::HeaderValue),
    NoResponse,
    InvalidJsonSchema,
    InvalidUrl,
    ClientError(Box<reqwest::Response>),
    ServerError(Box<reqwest::Response>),
    Bungie(crate::types::exceptions::PlatformErrorCodes),

    SerdeJson(serde_json::Error),
    Request(reqwest::Error),
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
    Io(std::io::Error),
    UrlParseError(url::ParseError),
}

impl Display for BungieApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for BungieApiError {}

impl From<serde_json::Error> for BungieApiError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJson(e)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for BungieApiError {
    fn from(value: reqwest::header::InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValue(value)
    }
}

impl From<reqwest::Error> for BungieApiError {
    fn from(value: reqwest::Error) -> Self {
        Self::Request(value)
    }
}

impl From<std::io::Error> for BungieApiError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<url::ParseError> for BungieApiError {
    fn from(value: url::ParseError) -> Self {
        Self::UrlParseError(value)
    }
}
