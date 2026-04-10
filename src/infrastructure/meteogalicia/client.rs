use std::sync::Arc;

use reqwest::{Method, StatusCode, header};
use tokio::sync::RwLock;

use crate::domain::error::Error;
use crate::domain::{error::Result, path::Path};
use crate::infrastructure::meteogalicia::cache::{Cache, CacheEntry};
use crate::infrastructure::meteogalicia::dtos::{GetResponse, ResponseBody};

pub struct Client {
    client: reqwest::Client,
    cache: Arc<RwLock<Cache>>,
    base_url: String,
    api_key: String,
}

impl Client {
    pub fn new(cache: Arc<RwLock<Cache>>, base_url: String, api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            cache,
            base_url,
            api_key,
        }
    }
}

impl Client {
    pub async fn get(&self, path: Path) -> Result<GetResponse<ResponseBody>> {
        let url = format!("{}/{}", self.get_base_url(), path.endpoint());

        let cache_key = format!("get:{}", path.to_string());

        if let Some(body) = {
            let cache = self.cache.read().await;
            cache.entries().get(&cache_key).cloned()
        }
        .map(|entry| entry.data)
        {
            return Ok(GetResponse { status: 200, body });
        }

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

        if status != StatusCode::OK {
            return Err(Error::BadRequest(body));
        }

        let Ok(body) = serde_json::from_str::<ResponseBody>(&body) else {
            return Err(Error::BadRequest(body));
        };

        {
            let mut cache = self.cache.write().await;
            cache.set(&cache_key, CacheEntry::from_data(body.clone()));
        }

        let entries = {
            let cache = self.cache.read().await;
            cache.entries().clone()
        };

        if let Err(e) = Cache::save_entries(entries).await {
            log::error!("Error saving in cache the response: {:?}", e);
        }

        Ok(GetResponse { status, body })
    }

    fn get_base_url(&self) -> String {
        format!("https://{}", self.base_url)
    }
}
