use std::sync::Arc;

use tokio::sync::{OnceCell, RwLock};

use crate::application::service::find_and_forecast_info::FindAndForecastInfo;
use crate::application::use_case::find_place::FindPlace;
use crate::application::use_case::get_forecast_info::GetForecastInfo;
use crate::domain::error::Result;
use crate::infrastructure::meteogalicia::cache::Cache;
use crate::infrastructure::meteogalicia::cached_repository::CachedMeteogaliciaRepository;
use crate::infrastructure::meteogalicia::client::Client;
use crate::infrastructure::meteogalicia::repository::MeteogaliciaRepository;

static REPOSITORY: OnceCell<Arc<CachedMeteogaliciaRepository<MeteogaliciaRepository>>> =
    OnceCell::const_new();

async fn get_repository() -> Arc<CachedMeteogaliciaRepository<MeteogaliciaRepository>> {
    let client = Client::new(
        std::env::var("BASE_URL").unwrap_or_default(),
        std::env::var("API_KEY").unwrap_or_default(),
    );
    let repository = MeteogaliciaRepository::new(client);
    let cache = Cache::builder(64).build().await.unwrap();
    let cache = Arc::new(RwLock::new(cache));
    Arc::new(CachedMeteogaliciaRepository::new(repository, cache))
}

pub async fn bootstrap_find_places()
-> Result<FindPlace<CachedMeteogaliciaRepository<MeteogaliciaRepository>>> {
    let meteogalicia_repository = REPOSITORY.get_or_init(get_repository).await.clone();
    let find_places = FindPlace::new(meteogalicia_repository);
    Ok(find_places)
}

pub async fn bootstrap_get_forecast_info()
-> Result<GetForecastInfo<CachedMeteogaliciaRepository<MeteogaliciaRepository>>> {
    let meteogalicia_repository = REPOSITORY.get_or_init(get_repository).await.clone();
    let get_forecast_info = GetForecastInfo::new(meteogalicia_repository);
    Ok(get_forecast_info)
}

pub async fn bootstrap_find_and_forecast_info()
-> Result<Arc<FindAndForecastInfo<CachedMeteogaliciaRepository<MeteogaliciaRepository>>>> {
    let find_places = bootstrap_find_places().await?;
    let get_forecast_info = bootstrap_get_forecast_info().await?;
    let batch_forecast_info = Arc::new(FindAndForecastInfo::new(find_places, get_forecast_info));
    Ok(batch_forecast_info)
}
