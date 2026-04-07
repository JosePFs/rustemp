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
        let url = format!("{}/{}", self.get_base_url(), path.endpoint());
        let response = self
            .client
            .request(Method::GET, url)
            .query(&path.as_query_params())
            .query(&[
                ("API_KEY", self.api_key.clone()),
                ("format", "application/json".to_string()),
            ])
            .header(header::ACCEPT, "application/json")
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .await?;

        let status = response.status().as_u16();
        let body = response.text().await?;

        let body: T = serde_json::from_str(&body)?;

        Ok(GetResponse { status, body })
    }

    fn get_base_url(&self) -> String {
        format!("https://{}", self.base_url)
    }
}
