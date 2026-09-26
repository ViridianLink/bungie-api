use std::error::Error;
use std::fmt::{self, Display, Formatter};

use reqwest::StatusCode;
use reqwest::header::{HeaderValue, InvalidHeaderValue};

use crate::types::exceptions::PlatformErrorCodes;

pub type Result<T> = std::result::Result<T, BungieApiError>;

#[derive(Debug)]
#[non_exhaustive]
pub enum BungieApiError {
    Http {
        status: StatusCode,
        body: String,
    },
    InvalidContentType(HeaderValue),
    Bungie {
        code: PlatformErrorCodes,
        status: String,
        message: String,
        throttle_seconds: i32,
    },
    MissingResponse,
    ManifestPathNotFound {
        locale: String,
        definition: &'static str,
    },
    InvalidUrl,
    SerdeJson(serde_json::Error),
    Request(reqwest::Error),
    InvalidHeaderValue(InvalidHeaderValue),
    UrlParse(url::ParseError),
}

impl Display for BungieApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http { status, .. } => write!(f, "HTTP error {status}"),
            Self::InvalidContentType(content_type) => {
                write!(f, "unexpected content type: {content_type:?}")
            },
            Self::Bungie { code, status, message, .. } => {
                write!(f, "Bungie error {} ({status}): {message}", code.code())
            },
            Self::MissingResponse => f.write_str("Bungie response had no payload"),
            Self::ManifestPathNotFound { locale, definition } => {
                write!(f, "manifest has no {definition} path for locale {locale:?}")
            },
            Self::InvalidUrl => f.write_str("could not build request URL"),
            Self::SerdeJson(e) => write!(f, "failed to parse JSON: {e}"),
            Self::Request(e) => write!(f, "request failed: {e}"),
            Self::InvalidHeaderValue(e) => write!(f, "invalid header value: {e}"),
            Self::UrlParse(e) => write!(f, "invalid URL: {e}"),
        }
    }
}

impl Error for BungieApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::SerdeJson(e) => Some(e),
            Self::Request(e) => Some(e),
            Self::InvalidHeaderValue(e) => Some(e),
            Self::UrlParse(e) => Some(e),
            Self::Http { .. }
            | Self::InvalidContentType(_)
            | Self::Bungie { .. }
            | Self::MissingResponse
            | Self::ManifestPathNotFound { .. }
            | Self::InvalidUrl => None,
        }
    }
}

impl From<serde_json::Error> for BungieApiError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJson(e)
    }
}

impl From<InvalidHeaderValue> for BungieApiError {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValue(value)
    }
}

impl From<reqwest::Error> for BungieApiError {
    fn from(value: reqwest::Error) -> Self {
        Self::Request(value)
    }
}

impl From<url::ParseError> for BungieApiError {
    fn from(value: url::ParseError) -> Self {
        Self::UrlParse(value)
    }
}
