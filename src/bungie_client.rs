use crate::{
    types::{exceptions::PlatformErrorCodes, response::BungieResponse},
    Error, Result,
};
use reqwest::{header::HeaderMap, Client, ClientBuilder, RequestBuilder, Url};
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
        default_headers.insert("X-API-Key", api_key.parse()?);

        let client = ClientBuilder::new()
            .default_headers(default_headers)
            .build()?;

        Ok(BungieClient { client })
    }

    pub async fn get<T>(&self, url: impl Into<Url>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = url.into();

        let reqwest = self.client.get(url);
        self.process_response::<T>(reqwest).await
    }

    pub async fn process_response<T>(&self, reqwest: RequestBuilder) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = reqwest.send().await?;

        if let Some(hv) = response.headers().get("Content-Type") {
            if !hv
                .to_str()
                .map(|s| s.starts_with("application/json"))
                .unwrap_or(false)
            {
                return Err(Error::InvalidContentType(hv.to_owned()));
            }
        }

        let deserialized = response.json::<BungieResponse<T>>().await?;
        match deserialized.error_code {
            PlatformErrorCodes::Success => Ok(deserialized.response),
            PlatformErrorCodes::Unknown(code) => {
                println!("Error Code: {}", code);
                Err(Error::Bungie(PlatformErrorCodes::Unknown(code)))
            }
            code => Err(Error::Bungie(code)),
        }
    }
}
