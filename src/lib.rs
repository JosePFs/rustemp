pub mod application;
pub mod bootstrap;
pub mod domain;
pub mod infrastructure;
pub mod interface;
pub mod tracing;

use crate::{
    bootstrap::{bootstrap_find_and_forecast_info, bootstrap_find_places},
    domain::{error::Result, forecast_info::ForecastInfo, location::Location, path::Path},
    interface::{find_args::FindArgs, forecast_args::ForecastArgs},
};

pub async fn run_find_and_forecast(forecast_args: ForecastArgs) -> Result<Vec<ForecastInfo>> {
    let batch_forecast_info = bootstrap_find_and_forecast_info().await?;

    let forecast_info = batch_forecast_info
        .execute(
            forecast_args
                .places
                .iter()
                .map(|place| place.clone().into())
                .collect(),
            forecast_args.types,
            forecast_args.start_time,
            forecast_args.end_time,
            forecast_args.data,
            forecast_args.lang.unwrap_or_default(),
        )
        .await?;

    Ok(forecast_info)
}

pub async fn run_find(find_args: FindArgs) -> Option<Location> {
    let find_places = bootstrap_find_places().await.ok()?;
    find_places
        .execute(
            Path::FindPlaces(
                find_args.place.clone().into(),
                find_args.types,
                find_args.lang.unwrap_or_default(),
            ),
            find_args.place.clone().into(),
        )
        .await
}
