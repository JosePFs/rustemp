use async_trait::async_trait;

use crate::domain::forecast_info::ForecastInfo;
use crate::domain::location::{Location, Place};
use crate::domain::path::Path;

#[async_trait]
pub trait ForecastRepository: Send + Sync {
    async fn find_location(&self, path: Path, place: Place) -> Option<Location>;
    async fn get_forecast_info(&self, path: Path) -> Option<ForecastInfo>;
}
