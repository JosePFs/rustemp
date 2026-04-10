use async_trait::async_trait;

use crate::{
    domain::{
        forecast_info::{
            Day, ForecastInfo, PlaceDays, PlaceDaysStatus,
            StringOrFloat as ForecastInfoStringOrFloat, Value, ValueDay,
        },
        location::{
            Geometry as DomainGeometry, GeometryCoordinates as DomainGeometryCoordinates,
            GeometryType as DomainGeometryType, Location, Name, Place,
        },
        municipality::Municipality,
        path::Path,
        repository::ForecastRepository,
        time::Time,
    },
    infrastructure::meteogalicia::{
        client::Client,
        dtos::{Feature, GeometryType, ResponseBody, StringOrFloat},
    },
};

pub struct MeteogaliciaRepository {
    client: Client,
}

impl MeteogaliciaRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl ForecastRepository for MeteogaliciaRepository {
    async fn find_location(&self, path: Path, place: Place) -> Option<Location> {
        let response = self.client.get(path).await.map_err(|e| {
            log::error!("Error finding location: {:?}", e);
            None::<Location>
        });

        let features = response.ok()?.body.features;
        let location_feature = features
            .iter()
            .find(|feature| {
                feature.properties.name.to_lowercase() == place.name.to_lowercase()
                    && feature.properties.municipality.to_lowercase()
                        == place.municipality.as_str().to_lowercase()
            })
            .or_else(|| {
                features.iter().find(|feature| {
                    feature
                        .properties
                        .name
                        .to_lowercase()
                        .contains(&place.name.to_lowercase())
                        && feature.properties.municipality.to_lowercase()
                            == place.municipality.as_str().to_lowercase()
                })
            })?;

        Some(location_feature.into())
    }

    async fn get_forecast_info(&self, path: Path) -> Option<ForecastInfo> {
        match self.client.get(path).await {
            Ok(response) => Some(response.body.into()),
            Err(e) => {
                log::error!("Error getting forecast info: {}", e.to_string());
                None
            }
        }
    }
}

impl From<&Feature> for Location {
    fn from(feature: &Feature) -> Self {
        Location::new(
            feature.properties.id.clone(),
            Place::new(
                Name::from(feature.properties.name.clone()),
                Municipality::from(feature.properties.municipality.clone()),
            ),
            DomainGeometry::new(
                feature.geometry.r#type.as_ref().into(),
                DomainGeometryCoordinates::new(
                    feature.geometry.coordinates.longitude,
                    feature.geometry.coordinates.latitude,
                ),
            ),
        )
    }
}

impl From<ResponseBody> for ForecastInfo {
    fn from(body: ResponseBody) -> Self {
        let places: Vec<PlaceDays> = body
            .features
            .iter()
            .map(|feature| PlaceDays {
                status: PlaceDaysStatus::Found,
                place: Place::new(
                    Name::from(feature.properties.name.clone()),
                    Municipality::from(feature.properties.municipality.clone()),
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
                                            Some(StringOrFloat::String(value)) => {
                                                ForecastInfoStringOrFloat::String(value.clone())
                                            }
                                            Some(StringOrFloat::Float(value)) => {
                                                ForecastInfoStringOrFloat::Float(value.clone())
                                            }
                                            None if value.direction_value.is_some()
                                                && value.module_value.is_some() =>
                                            {
                                                ForecastInfoStringOrFloat::FloatAndFloat(
                                                    value.module_value.unwrap_or(0.0),
                                                    value.direction_value.unwrap_or(0.0),
                                                )
                                            }
                                            None => {
                                                ForecastInfoStringOrFloat::String("".to_string())
                                            }
                                        },
                                    })
                                    .collect(),
                                units: match &variable.units {
                                    Some(units) => units.clone(),
                                    None => match &variable.module_units {
                                        Some(units) => format!(
                                            "{}-{}",
                                            units.clone(),
                                            &variable
                                                .direction_units
                                                .clone()
                                                .unwrap_or("".to_string())
                                        ),
                                        None => "".to_string(),
                                    },
                                },
                            })
                            .collect(),
                    })
                    .collect(),
                geometry: DomainGeometry::new(
                    match feature.geometry.r#type {
                        GeometryType::Point => DomainGeometryType::Point,
                        GeometryType::LineString => DomainGeometryType::LineString,
                        GeometryType::Polygon => DomainGeometryType::Polygon,
                    },
                    DomainGeometryCoordinates::new(
                        feature.geometry.coordinates.longitude,
                        feature.geometry.coordinates.latitude,
                    ),
                ),
            })
            .collect();
        ForecastInfo { places }
    }
}
