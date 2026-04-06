use reqwest::{Method, header};
use serde::de::DeserializeOwned;

use crate::domain::{error::Result, path::Path};
use crate::infrastructure::meteogalicia::dtos::GetResponse;

pub struct Client {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl Client {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
            api_key,
        }
    }
}

impl Client {
    pub async fn get<T: DeserializeOwned>(&self, path: Path) -> Result<GetResponse<T>> {
        let response = self
            .client
            .request(Method::GET, self.get_url(path.clone()))
            .header(header::ACCEPT, "application/json")
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .send()
            .await?;

        let status = response.status().as_u16();
        let body = response.text().await?;
        let body: T = serde_json::from_str(&body)?;

        Ok(GetResponse { status, body })
    }

    fn get_url(&self, path: Path) -> String {
        format!("https://{}/{}", self.base_url, path.as_str(&self.api_key))
    }
}
