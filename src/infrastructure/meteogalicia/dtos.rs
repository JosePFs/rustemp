use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};

use crate::domain::{
    forecast_info::ForecastInfo, forecast_info::StringOrFloat as ForecastInfoStringOrFloat,
    location::Location,
};

#[derive(Debug, Deserialize)]
pub struct FindPlacesBody {
    pub features: Vec<Feature>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GeometryType {
    #[serde(rename = "Point")]
    Point,
    #[serde(rename = "LineString")]
    LineString,
    #[serde(rename = "Polygon")]
    Polygon,
}

impl From<&str> for GeometryType {
    fn from(value: &str) -> Self {
        match value {
            "Point" => GeometryType::Point,
            "LineString" => GeometryType::LineString,
            "Polygon" => GeometryType::Polygon,
            _ => GeometryType::Point,
        }
    }
}

impl From<GeometryType> for &str {
    fn from(value: GeometryType) -> Self {
        match value {
            GeometryType::Point => "Point",
            GeometryType::LineString => "LineString",
            GeometryType::Polygon => "Polygon",
        }
    }
}

impl AsRef<str> for GeometryType {
    fn as_ref(&self) -> &str {
        match self {
            GeometryType::Point => "Point",
            GeometryType::LineString => "LineString",
            GeometryType::Polygon => "Polygon",
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GeometryCoordinates {
    pub longitude: f64,
    pub latitude: f64,
}

impl<'de> Deserialize<'de> for GeometryCoordinates {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let [longitude, latitude] = <[f64; 2]>::deserialize(deserializer)?;
        Ok(Self {
            longitude,
            latitude,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    pub r#type: GeometryType,
    pub coordinates: GeometryCoordinates,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
    pub properties: Properties,
    pub geometry: Geometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    pub name: String,
    pub municipality: String,
    pub id: String,
    #[serde(default)]
    pub days: Vec<Day>,
    pub units: Option<String>,
    #[serde(rename = "moduleUnits")]
    pub module_units: Option<String>,
    #[serde(rename = "directionUnits")]
    pub direction_units: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Day {
    #[serde(rename = "timePeriod")]
    pub time_period: TimePeriod,
    pub variables: Vec<Variable>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimePeriod {
    pub begin: TimeInstant,
    pub end: TimeInstant,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeInstant {
    #[serde(rename = "timeInstant")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetForecastInfoBody {
    pub features: Vec<Feature>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub values: Vec<Value>,
    pub units: Option<String>,
    #[serde(rename = "moduleUnits")]
    pub module_units: Option<String>,
    #[serde(rename = "directionUnits")]
    pub direction_units: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    #[serde(rename = "timeInstant")]
    pub time_instant: ValueTimeInstant,
    pub value: Option<StringOrFloat>,
    #[serde(rename = "directionValue")]
    pub direction_value: Option<f64>,
    #[serde(rename = "moduleValue")]
    pub module_value: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueTimeInstant(pub String);

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrFloat {
    String(String),
    Float(f64),
}

#[derive(Debug)]
pub struct GetResponse<T: DeserializeOwned> {
    pub status: u16,
    pub body: T,
}

#[derive(Debug, Serialize)]
pub struct ForecastOutput {
    pub places: Vec<PlaceOutput>,
}

#[derive(Debug, Serialize)]
pub struct PlaceOutput {
    pub name: String,
    pub municipality: String,
    pub days: Vec<DayOutput>,
    pub units: HashMap<String, String>,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct DayOutput {
    pub date: String,
    pub values: HashMap<String, Vec<ValueOutput>>,
}

#[derive(Debug, Serialize)]
pub struct ValueOutput {
    pub time: String,
    pub value: serde_json::Value,
}

impl From<ForecastInfo> for ForecastOutput {
    fn from(domain: ForecastInfo) -> Self {
        let places = domain
            .places
            .into_iter()
            .map(|place_days| {
                let days = place_days
                    .days
                    .iter()
                    .map(|day| {
                        let mut values_map: HashMap<String, Vec<ValueOutput>> = HashMap::new();

                        for value_day in &day.values {
                            let values = value_day
                                .values
                                .iter()
                                .map(|v| ValueOutput {
                                    time: v.time.to_string(),
                                    value: match &v.value {
                                        ForecastInfoStringOrFloat::Float(f) => {
                                            serde_json::json!(f)
                                        }
                                        ForecastInfoStringOrFloat::String(s) => {
                                            serde_json::json!(s)
                                        }
                                        ForecastInfoStringOrFloat::FloatAndFloat(f1, f2) => {
                                            serde_json::json!({ "speed": f1, "direction": f2 })
                                        }
                                    },
                                })
                                .collect();

                            values_map.insert(value_day.name.clone(), values);
                        }

                        DayOutput {
                            date: day.begin.to_string()[0..10].to_string(),
                            values: values_map,
                        }
                    })
                    .collect();

                PlaceOutput {
                    name: place_days.place.name.to_string(),
                    municipality: place_days.place.municipality.to_string(),
                    days,
                    units: place_days
                        .days
                        .iter()
                        .flat_map(|day| day.values.iter())
                        .map(|value_day| (value_day.name.clone(), value_day.units.clone()))
                        .collect::<HashMap<String, String>>(),
                    status: place_days.status.to_string(),
                }
            })
            .collect();

        ForecastOutput { places }
    }
}

#[derive(Debug, Serialize)]
pub struct LocationOutput {
    pub id: String,
    pub name: String,
    pub municipality: String,
}

#[derive(Debug, Serialize)]
pub struct LocationNotFoundOutput {
    pub name: String,
    pub municipality: String,
    pub error: String,
}

impl From<Location> for LocationOutput {
    fn from(location: Location) -> Self {
        LocationOutput {
            id: location.id,
            name: location.place.name.to_string(),
            municipality: location.place.municipality.to_string(),
        }
    }
}
