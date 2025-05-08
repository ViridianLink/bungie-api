use crate::{
    types::{exceptions::PlatformErrorCodes, response::BungieResponse},
    Error, Result,
};
use reqwest::{
    header::{self, HeaderMap},
    Client, ClientBuilder, IntoUrl, Response,
};
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
        const NAME: &str = env!("CARGO_PKG_NAME");
        const VERSION: &str = env!("CARGO_PKG_VERSION");

        let mut default_headers = HeaderMap::new();
        default_headers.insert("X-API-Key", api_key.parse().unwrap());
        default_headers.insert(
            header::USER_AGENT,
            format!("{NAME}/{VERSION}").parse().unwrap(),
        );

        let client = ClientBuilder::new()
            .default_headers(default_headers)
            .build()
            .unwrap();

        Ok(BungieClient { client })
    }

    pub async fn get<T: DeserializeOwned>(&self, url: impl IntoUrl) -> Result<T> {
        let reqwest = self.client.get(url);
        let mut res = reqwest.send().await.unwrap();
        res = Self::validate_status(res)?;
        res = Self::validate_content_type(res)?;
        let text = res.text().await.unwrap();
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

    fn validate_status(response: Response) -> Result<Response> {
        match response.status().as_u16() {
            // Information
            100..200 => Ok(response),
            // Success
            200..300 => Ok(response),
            // Redirection
            300..400 => Ok(response),
            // Client Error
            400..500 => Err(Error::ClientError(response)),
            // Server Error
            500..600 => Err(Error::ServerError(response)),
            _ => Ok(response),
        }
    }

    fn validate_content_type(response: Response) -> Result<Response> {
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
