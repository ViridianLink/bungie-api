use crate::{
    types::{exceptions::PlatformErrorCodes, response::BungieResponse},
    Error, Result,
};
use reqwest::{header::HeaderMap, Client, ClientBuilder, IntoUrl, RequestBuilder, Response};
use serde::de::DeserializeOwned;

pub struct BungieClientBuilder {
    api_key: String,
}

impl BungieClientBuilder {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
        }
    }

    pub fn build(self) -> Result<BungieClient> {
        BungieClient::new(self.api_key)
    }
}

pub struct BungieClient {
    pub(crate) client: Client,
}

impl BungieClient {
    pub fn new(api_key: String) -> Result<Self> {
        let mut default_headers = HeaderMap::new();
        default_headers.insert("X-API-Key", api_key.parse().unwrap());

        let client = ClientBuilder::new()
            .default_headers(default_headers)
            .build()
            .unwrap();

        Ok(BungieClient { client })
    }

    pub async fn get<T: DeserializeOwned>(&self, url: impl IntoUrl) -> Result<T> {
        let reqwest = self.client.get(url);
        let response = Self::validate_content_type(reqwest).await?;
        let text = response.text().await.unwrap();
        match serde_json::from_str::<T>(&text) {
            Ok(json) => Ok(json),
            Err(e) => {
                #[cfg(test)]
                std::fs::write("error.json", text).unwrap();
                Err(e.into())
            }
        }
    }

    pub async fn get_bungie_response<T: DeserializeOwned>(&self, url: impl IntoUrl) -> Result<T> {
        let res = self.get::<BungieResponse<T>>(url).await?;
        Self::handle_bungie_response(res).await
    }

    async fn validate_content_type(reqwest: RequestBuilder) -> Result<Response> {
        let response = reqwest.send().await.unwrap();

        if let Some(hv) = response.headers().get("Content-Type") {
            if !hv
                .to_str()
                .map(|s| s.starts_with("application/json"))
                .unwrap_or(false)
            {
                return Err(Error::InvalidContentType(hv.to_owned()));
            }
        }

        Ok(response)
    }

    pub async fn handle_bungie_response<T>(de: BungieResponse<T>) -> Result<T> {
        match de.error_code {
            PlatformErrorCodes::Success => Ok(de.response),
            PlatformErrorCodes::Unknown(code) => {
                println!("Error Code: {}", code);
                Err(Error::Bungie(PlatformErrorCodes::Unknown(code)))
            }
            code => Err(Error::Bungie(code)),
        }
    }
}
