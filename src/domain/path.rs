use std::fmt::{Display, Formatter};

use crate::domain::{
    lang::Lang,
    location::{Location, Place, PlaceType},
    parameter::Parameter,
    time::Time,
};

#[derive(Debug, Clone)]
pub enum Path {
    FindPlaces(Place, Vec<PlaceType>, Lang),
    GetForecastInfo(Vec<Location>, Vec<Parameter>, Time, Time, Lang),
}

impl Path {
    pub fn endpoint(&self) -> String {
        match self {
            Path::FindPlaces(_, _, _) => "findPlaces".to_string(),
            Path::GetForecastInfo(_, _, _, _, _) => "getNumericForecastInfo".to_string(),
        }
    }

    pub fn as_query_params(&self) -> Vec<(String, String)> {
        match self {
            Path::FindPlaces(place, types, lang) => {
                vec![
                    ("location".to_string(), place.name.to_string()),
                    (
                        "types".to_string(),
                        types
                            .iter()
                            .map(|t| t.to_string())
                            .collect::<Vec<String>>()
                            .join(","),
                    ),
                    ("lang".to_string(), lang.to_string()),
                ]
            }
            Path::GetForecastInfo(location_ids, variables, start_time, end_time, lang) => {
                vec![
                    (
                        "locationIds".to_string(),
                        location_ids
                            .iter()
                            .map(|l| l.to_string())
                            .collect::<Vec<String>>()
                            .join(","),
                    ),
                    (
                        "variables".to_string(),
                        variables
                            .iter()
                            .map(|v| v.as_str())
                            .collect::<Vec<&str>>()
                            .join(","),
                    ),
                    ("startTime".to_string(), start_time.to_string()),
                    ("endTime".to_string(), end_time.to_string()),
                    ("lang".to_string(), lang.to_string()),
                ]
            }
        }
    }

    pub fn lang(&self) -> Lang {
        match self {
            Path::FindPlaces(_, _, lang) => lang.clone(),
            Path::GetForecastInfo(_, _, _, _, lang) => lang.clone(),
        }
    }

    fn to_string(&self) -> String {
        format!(
            "{}:{}",
            self.endpoint(),
            self.as_query_params()
                .iter()
                .map(|(key, value)| format!("{}={}", key, value))
                .collect::<Vec<String>>()
                .join("&"),
        )
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
