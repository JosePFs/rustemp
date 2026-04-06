use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    domain::{
        forecast_info::{
            Day, ForecastInfo, PlaceDays, PlaceDaysStatus,
            StringOrFloat as ForecastInfoStringOrFloat, Value, ValueDay,
        },
        location::{Location, Name, Place},
        municipality::Municipality,
        path::Path,
        repository::ForecastRepository,
        time::Time,
    },
    infrastructure::meteogalicia::{
        client::Client,
        dtos::{FindPlacesBody, GetForecastInfoBody, StringOrFloat},
    },
};

pub struct MeteogaliciaRepository {
    client: Arc<Client>,
}

impl MeteogaliciaRepository {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl ForecastRepository for MeteogaliciaRepository {
    async fn find_location(&self, path: Path, place: Place) -> Option<Location> {
        let response = self
            .client
            .get::<FindPlacesBody>(path)
            .await
            .map_err(|_| None::<Location>);
        let location = response.ok()?.body.features.into_iter().find(|feature| {
            feature.properties.name.to_lowercase() == place.name.to_lowercase()
                && feature.properties.municipality.to_lowercase()
                    == place.municipality.as_str().to_lowercase()
        })?;

        Some(Location::new(
            location.properties.id,
            Place::new(
                Name::from_str(&location.properties.name),
                Municipality::from(location.properties.municipality),
            ),
        ))
    }

    async fn get_forecast_info(&self, path: Path) -> Option<ForecastInfo> {
        match self.client.get::<GetForecastInfoBody>(path).await {
            Ok(response) => {
                let forecast_info: ForecastInfo = response.body.into();
                Some(forecast_info)
            }
            Err(_) => None,
        }
    }
}

impl From<GetForecastInfoBody> for ForecastInfo {
    fn from(body: GetForecastInfoBody) -> Self {
        let places: Vec<PlaceDays> = body
            .features
            .into_iter()
            .map(|feature| PlaceDays {
                status: PlaceDaysStatus::Found,
                place: Place::new(
                    Name::from(feature.properties.name),
                    Municipality::from(feature.properties.municipality),
                ),
                days: feature
                    .properties
                    .days
                    .iter()
                    .map(|day| Day {
                        begin: Time::from(day.time_period.begin.value.clone()),
                        end: Time::from(day.time_period.end.value.clone()),
                        values: day
                            .variables
                            .iter()
                            .map(|variable| ValueDay {
                                name: variable.name.clone(),
                                values: variable
                                    .values
                                    .iter()
                                    .map(|value| Value {
                                        time: Time::from(value.time_instant.0.clone()),
                                        value: match &value.value {
                                            StringOrFloat::String(value) => {
                                                ForecastInfoStringOrFloat::String(value.clone())
                                            }
                                            StringOrFloat::Float(value) => {
                                                ForecastInfoStringOrFloat::Float(value.clone())
                                            }
                                        },
                                    })
                                    .collect(),
                            })
                            .collect(),
                    })
                    .collect(),
            })
            .collect();
        ForecastInfo { places }
    }
}
