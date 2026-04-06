use std::sync::{Arc, LazyLock};

use crate::application::service::find_and_forecast_info::FindAndForecastInfo;
use crate::application::use_case::find_place::FindPlace;
use crate::application::use_case::get_forecast_info::GetForecastInfo;
use crate::domain::error::Result;
use crate::infrastructure::meteogalicia::client::Client;
use crate::infrastructure::meteogalicia::repository::MeteogaliciaRepository;

static CLIENT: LazyLock<Arc<Client>> = LazyLock::new(|| {
    Arc::new(Client::new(
        std::env::var("BASE_URL").unwrap_or_default(),
        std::env::var("API_KEY").unwrap_or_default(),
    ))
});

static REPOSITORY: LazyLock<Arc<MeteogaliciaRepository>> = LazyLock::new(|| {
    let client = &*CLIENT;
    Arc::new(MeteogaliciaRepository::new(client.clone()))
});

pub fn bootstrap_find_places() -> Result<FindPlace<MeteogaliciaRepository>> {
    let meteogalicia_repository = &*REPOSITORY;
    let find_places = FindPlace::new(meteogalicia_repository.clone());
    Ok(find_places)
}

pub fn bootstrap_get_forecast_info() -> Result<GetForecastInfo<MeteogaliciaRepository>> {
    let meteogalicia_repository = &*REPOSITORY;
    let get_forecast_info = GetForecastInfo::new(meteogalicia_repository.clone());
    Ok(get_forecast_info)
}

pub fn bootstrap_find_and_forecast_info() -> Result<Arc<FindAndForecastInfo<MeteogaliciaRepository>>>
{
    let find_places = bootstrap_find_places()?;
    let get_forecast_info = bootstrap_get_forecast_info()?;
    let batch_forecast_info = Arc::new(FindAndForecastInfo::new(find_places, get_forecast_info));
    Ok(batch_forecast_info)
}
