use std::fmt::{Display, Formatter};

use crate::domain::location::Place;
use crate::domain::time::Time;

#[derive(Debug, Clone)]
pub enum StringOrFloat {
    String(String),
    Float(f64),
}

#[derive(Debug, Clone)]
pub struct Value {
    pub time: Time,
    pub value: StringOrFloat,
}

#[derive(Debug, Clone)]
pub struct ValueDay {
    pub name: String,
    pub values: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct Day {
    pub begin: Time,
    pub end: Time,
    pub values: Vec<ValueDay>,
}

#[derive(Debug, Clone)]
pub enum PlaceDaysStatus {
    Found,
    LocationNotFound,
    ForecastInfoNotFound,
}

impl Display for PlaceDaysStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PlaceDaysStatus::Found => "Found",
                PlaceDaysStatus::LocationNotFound => "LocationNotFound",
                PlaceDaysStatus::ForecastInfoNotFound => "ForecastInfoNotFound",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct PlaceDays {
    pub place: Place,
    pub days: Vec<Day>,
    pub status: PlaceDaysStatus,
}

impl PlaceDays {
    pub fn new(place: Place, days: Vec<Day>, status: PlaceDaysStatus) -> Self {
        Self {
            place,
            days,
            status,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ForecastInfo {
    pub places: Vec<PlaceDays>,
}

impl ForecastInfo {
    pub fn new(places: Vec<PlaceDays>) -> Self {
        Self { places }
    }
}
