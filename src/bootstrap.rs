use std::sync::Arc;

use tokio::sync::{OnceCell, RwLock};

use crate::application::service::find_and_forecast_info::FindAndForecastInfo;
use crate::application::use_case::find_place::FindPlace;
use crate::application::use_case::get_forecast_info::GetForecastInfo;
use crate::domain::error::Result;
use crate::infrastructure::meteogalicia::cache::Cache;
use crate::infrastructure::meteogalicia::client::Client;
use crate::infrastructure::meteogalicia::repository::MeteogaliciaRepository;

static REPOSITORY: OnceCell<Arc<MeteogaliciaRepository>> = OnceCell::const_new();

async fn get_repository() -> Arc<MeteogaliciaRepository> {
    let cache = Cache::builder(64).build().await.unwrap();
    let cache = Arc::new(RwLock::new(cache));
    let client = Client::new(
        cache,
        std::env::var("BASE_URL").unwrap_or_default(),
        std::env::var("API_KEY").unwrap_or_default(),
    );
    Arc::new(MeteogaliciaRepository::new(client))
}

pub async fn bootstrap_find_places() -> Result<FindPlace<MeteogaliciaRepository>> {
    let meteogalicia_repository = REPOSITORY.get_or_init(get_repository).await.clone();
    let find_places = FindPlace::new(meteogalicia_repository);
    Ok(find_places)
}

pub async fn bootstrap_get_forecast_info() -> Result<GetForecastInfo<MeteogaliciaRepository>> {
    let meteogalicia_repository = REPOSITORY.get_or_init(get_repository).await.clone();
    let get_forecast_info = GetForecastInfo::new(meteogalicia_repository);
    Ok(get_forecast_info)
}

pub async fn bootstrap_find_and_forecast_info()
-> Result<Arc<FindAndForecastInfo<MeteogaliciaRepository>>> {
    let find_places = bootstrap_find_places().await?;
    let get_forecast_info = bootstrap_get_forecast_info().await?;
    let batch_forecast_info = Arc::new(FindAndForecastInfo::new(find_places, get_forecast_info));
    Ok(batch_forecast_info)
}
