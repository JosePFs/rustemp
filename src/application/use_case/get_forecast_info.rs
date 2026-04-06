use std::sync::Arc;

use crate::domain::{forecast_info::ForecastInfo, path::Path, repository::ForecastRepository};

pub struct GetForecastInfo<R: ForecastRepository> {
    repository: Arc<R>,
}

impl<R: ForecastRepository> GetForecastInfo<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

impl<R: ForecastRepository> GetForecastInfo<R> {
    pub async fn execute(&self, path: Path) -> Option<ForecastInfo> {
        self.repository.get_forecast_info(path).await
    }
}
