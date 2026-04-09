use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
    domain::{
        forecast_info::ForecastInfo,
        location::{Location, Place},
        path::Path,
        repository::ForecastRepository,
    },
    infrastructure::meteogalicia::{
        cache::{Cache, CacheEntry},
        dtos::Feature,
    },
};

pub struct CachedMeteogaliciaRepository<R: ForecastRepository> {
    repository: R,
    cache: Arc<RwLock<Cache>>,
}

impl<R: ForecastRepository> CachedMeteogaliciaRepository<R> {
    pub fn new(repository: R, cache: Arc<RwLock<Cache>>) -> Self {
        Self { repository, cache }
    }
}

#[async_trait]
impl<R: ForecastRepository> ForecastRepository for CachedMeteogaliciaRepository<R> {
    async fn find_location(&self, path: Path, place: Place) -> Option<Location> {
        let cache_key = format!(
            "find_location:{}",
            path.as_query_params()
                .iter()
                .map(|(key, value)| format!("{}={}", key, value))
                .collect::<Vec<String>>()
                .join("&")
        );

        if let Some(entry) = {
            let cache = self.cache.read().await;
            cache.entries().get(&cache_key).cloned()
        }
        .map(|entry| entry.data.into())
        {
            return Some(entry);
        }

        let location = self.repository.find_location(path, place).await?;

        let feature = Feature::from(&location);

        {
            let mut cache = self.cache.write().await;
            cache.set(&cache_key, CacheEntry::from_data(feature));
        }

        let entries = {
            let cache = self.cache.read().await;
            cache.entries().clone()
        };

        if let Err(e) = Cache::save_entries(entries).await {
            log::error!("Error saving cache entries: {:?}", e);
        }

        Some(location)
    }

    async fn get_forecast_info(&self, path: Path) -> Option<ForecastInfo> {
        self.repository
            .get_forecast_info(path)
            .await
            .map(|response| response.into())
    }
}
