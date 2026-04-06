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
    pub fn as_str(&self, api_key: &str) -> String {
        match self {
            Path::FindPlaces(place, types, lang) => {
                format!(
                    "findPlaces?location={}&types={}&API_KEY={}&lang={}&format=application/json",
                    place.name,
                    types
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                    api_key,
                    lang
                )
            }
            Path::GetForecastInfo(location_ids, variables, start_time, end_time, lang) => {
                format!(
                    "getNumericForecastInfo?locationIds={}&variables={}&startTime={}&endTime={}&API_KEY={}&lang={}&format=application/json",
                    location_ids
                        .iter()
                        .map(|l| l.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                    variables
                        .iter()
                        .map(|v| v.as_str())
                        .collect::<Vec<&str>>()
                        .join(","),
                    start_time,
                    end_time,
                    api_key,
                    lang
                )
            }
        }
    }

    pub fn lang(&self) -> Lang {
        match self {
            Path::FindPlaces(_, _, lang) => lang.clone(),
            Path::GetForecastInfo(_, _, _, _, lang) => lang.clone(),
        }
    }
}
