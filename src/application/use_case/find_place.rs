use std::sync::Arc;

use crate::domain::{
    location::{Location, Place},
    path::Path,
    repository::ForecastRepository,
};

pub struct FindPlace<R: ForecastRepository> {
    repository: Arc<R>,
}

impl<R: ForecastRepository> FindPlace<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

impl<R: ForecastRepository> FindPlace<R> {
    pub async fn execute(&self, path: Path, place: Place) -> Option<Location> {
        self.repository.find_location(path, place).await
    }
}
