use crate::{
    domain::{error::Result, forecast_info::ForecastInfo, location::Location},
    infrastructure::meteogalicia::dtos::{ForecastOutput, LocationNotFoundOutput, LocationOutput},
    interface::cli::PlacePair,
};

pub fn find_and_forecast_to_json(forecast_infos: Vec<ForecastInfo>) -> Result<String> {
    let json = serde_json::to_string_pretty(
        &forecast_infos
            .into_iter()
            .map(|forecast_info| forecast_info.into())
            .collect::<Vec<ForecastOutput>>(),
    )?;
    Ok(json)
}

pub fn find_location_to_json(location: Option<Location>, place: PlacePair) -> Result<String> {
    let json = match location {
        Some(location) => {
            let location_output: LocationOutput = location.into();
            serde_json::to_string_pretty(&location_output)?
        }
        None => {
            let location_not_found_output: LocationNotFoundOutput = LocationNotFoundOutput {
                name: place.location,
                municipality: place.municipality,
                error: "Location not found".to_string(),
            };
            serde_json::to_string_pretty(&location_not_found_output)?
        }
    };
    Ok(json)
}
