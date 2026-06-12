use reqwest::header::HeaderMap;
use reqwest::{Client, ClientBuilder, IntoUrl, Response, header};
use serde::de::DeserializeOwned;

use crate::types::exceptions::PlatformErrorCodes;
use crate::types::response::BungieResponse;
use crate::{BungieApiError, Result};

pub struct BungieClientBuilder {
    api_key: String,
}

impl BungieClientBuilder {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self { api_key: api_key.into() }
    }

    pub fn build(self) -> Result<BungieClient> {
        BungieClient::new(&self.api_key)
    }
}

pub struct BungieClient {
    pub(crate) client: Client,
}

impl BungieClient {
    pub fn new(api_key: &str) -> Result<Self> {
        const NAME: &str = env!("CARGO_PKG_NAME");
        const VERSION: &str = env!("CARGO_PKG_VERSION");

        let mut default_headers = HeaderMap::new();
        default_headers.insert("X-API-Key", api_key.parse()?);
        default_headers
            .insert(header::USER_AGENT, format!("{NAME}/{VERSION}").parse()?);

        let client =
            ClientBuilder::new().default_headers(default_headers).build()?;

        Ok(Self { client })
    }

    pub async fn get<T: DeserializeOwned>(&self, url: impl IntoUrl) -> Result<T> {
        let reqwest = self.client.get(url);
        let mut res = reqwest.send().await?;
        res = Self::validate_status(res)?;
        res = Self::validate_content_type(res)?;
        let text = res.text().await?;
        match serde_json::from_str::<T>(&text) {
            Ok(json) => Ok(json),
            Err(e) => {
                #[cfg(test)]
                std::fs::write("error.json", text)?;
                Err(e.into())
            },
        }
    }

    pub async fn get_bungie_response<T: DeserializeOwned>(
        &self,
        url: impl IntoUrl,
    ) -> Result<T> {
        let res = self.get::<BungieResponse<T>>(url).await?;
        Self::handle_bungie_response(res)
    }

    fn validate_status(response: Response) -> Result<Response> {
        match response.status().as_u16() {
            100..400 => Ok(response),
            // Client Error
            400..500 => Err(BungieApiError::ClientError(Box::new(response))),
            // Server Error
            500..600 => Err(BungieApiError::ServerError(Box::new(response))),
            code => {
                println!("unrecognized code: {code}");
                Ok(response)
            },
        }
    }

    fn validate_content_type(response: Response) -> Result<Response> {
        if let Some(hv) = response.headers().get("Content-Type")
            && !hv.to_str().is_ok_and(|s| s.starts_with("application/json"))
        {
            return Err(BungieApiError::InvalidContentType(hv.to_owned()));
        }

        Ok(response)
    }

    pub fn handle_bungie_response<T>(de: BungieResponse<T>) -> Result<T> {
        #[expect(
            unreachable_patterns,
            reason = "PlatformErrorCodes is non exhaustive"
        )]
        match de.error_code {
            PlatformErrorCodes::Success => Ok(de.response),
            PlatformErrorCodes::Unknown(code) => {
                println!("Error Code: {code}");
                Err(BungieApiError::Bungie(PlatformErrorCodes::Unknown(code)))
            },
            code => Err(BungieApiError::Bungie(code)),
        }
    }
}
