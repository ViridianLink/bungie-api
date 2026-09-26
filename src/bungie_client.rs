use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder, IntoUrl, RequestBuilder, StatusCode};
use serde::Serialize;
use serde::de::{DeserializeOwned, IgnoredAny};
use url::Url;

use crate::types::exceptions::PlatformErrorCodes;
use crate::types::response::BungieResponse;
use crate::{BungieApiError, Result};

pub(crate) const BUNGIE_URL: &str = "https://www.bungie.net";
const PLATFORM_URL: &str = "https://www.bungie.net/Platform/";

pub struct BungieClientBuilder {
    api_key: String,
    user_agent: Option<String>,
}

impl BungieClientBuilder {
    #[must_use]
    pub fn new(api_key: impl Into<String>) -> Self {
        Self { api_key: api_key.into(), user_agent: None }
    }

    #[must_use]
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    pub fn build(self) -> Result<BungieClient> {
        const DEFAULT_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

        let mut api_key = HeaderValue::from_str(&self.api_key)?;
        api_key.set_sensitive(true);

        let user_agent = match self.user_agent {
            Some(user_agent) => HeaderValue::from_str(&user_agent)?,
            None => HeaderValue::from_static(DEFAULT_USER_AGENT),
        };

        let mut default_headers = HeaderMap::new();
        default_headers.insert("X-API-Key", api_key);
        default_headers.insert(header::USER_AGENT, user_agent);

        let client =
            ClientBuilder::new().default_headers(default_headers).build()?;

        Ok(BungieClient { client, platform_url: Url::parse(PLATFORM_URL)? })
    }
}

#[derive(Debug, Clone)]
pub struct BungieClient {
    pub(crate) client: Client,
    platform_url: Url,
}

impl BungieClient {
    pub fn new(api_key: &str) -> Result<Self> {
        BungieClientBuilder::new(api_key).build()
    }

    pub async fn get<T: DeserializeOwned>(&self, url: impl IntoUrl) -> Result<T> {
        let (status, content_type, body) = Self::send(self.client.get(url)).await?;

        if !status.is_success() {
            return Err(Self::http_error(status, body.as_ref()));
        }
        Self::validate_content_type(content_type.as_ref())?;

        Self::parse_json(body.as_ref())
    }

    pub async fn get_bungie_response<T: DeserializeOwned>(
        &self,
        url: impl IntoUrl,
    ) -> Result<T> {
        Self::send_bungie(self.client.get(url)).await
    }

    pub async fn post_bungie_response<T, B>(
        &self,
        url: impl IntoUrl,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        Self::send_bungie(self.client.post(url).json(body)).await
    }

    pub(crate) fn platform_url<I>(&self, segments: I) -> Result<Url>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let mut url = self.platform_url.clone();
        url.path_segments_mut()
            .map_err(|()| BungieApiError::InvalidUrl)?
            .pop_if_empty()
            .extend(segments)
            .push("");
        Ok(url)
    }

    async fn send(
        request: RequestBuilder,
    ) -> Result<(StatusCode, Option<HeaderValue>, impl AsRef<[u8]>)> {
        let response = request.send().await?;
        let status = response.status();
        let content_type = response.headers().get(header::CONTENT_TYPE).cloned();
        let body = response.bytes().await?;
        Ok((status, content_type, body))
    }

    async fn send_bungie<T: DeserializeOwned>(request: RequestBuilder) -> Result<T> {
        let (status, content_type, body) = Self::send(request).await?;
        let body = body.as_ref();

        if !status.is_success() {
            // Bungie usually explains failures in its JSON envelope.
            return Err(
                match serde_json::from_slice::<BungieResponse<IgnoredAny>>(body) {
                    Ok(envelope)
                        if envelope.error_code != PlatformErrorCodes::Success =>
                    {
                        envelope.into_error()
                    },
                    _ => Self::http_error(status, body),
                },
            );
        }
        Self::validate_content_type(content_type.as_ref())?;

        Self::parse_json::<BungieResponse<T>>(body)?.into_result()
    }

    fn parse_json<T: DeserializeOwned>(body: &[u8]) -> Result<T> {
        serde_json::from_slice(body).map_err(|e| {
            // Keep the payload around to make schema mismatches easy to debug.
            #[cfg(test)]
            let _ = std::fs::write("error.json", body);
            e.into()
        })
    }

    fn http_error(status: StatusCode, body: &[u8]) -> BungieApiError {
        BungieApiError::Http {
            status,
            body: String::from_utf8_lossy(body).into_owned(),
        }
    }

    fn validate_content_type(content_type: Option<&HeaderValue>) -> Result<()> {
        match content_type {
            Some(hv)
                if !hv.to_str().is_ok_and(|s| s.starts_with("application/json")) =>
            {
                Err(BungieApiError::InvalidContentType(hv.clone()))
            },
            _ => Ok(()),
        }
    }
}
