use futures::future::join_all;

use crate::{
    application::use_case::{find_place::FindPlace, get_forecast_info::GetForecastInfo},
    domain::{
        error::Result,
        forecast_info::{ForecastInfo, PlaceDays, PlaceDaysStatus},
        lang::Lang,
        location::{Location, Place, PlaceType},
        parameter::Parameter,
        path::Path,
        repository::ForecastRepository,
        time::Time,
    },
};

pub struct FindAndForecastInfo<R: ForecastRepository> {
    find_places: FindPlace<R>,
    get_forecast_info: GetForecastInfo<R>,
}

impl<R: ForecastRepository> FindAndForecastInfo<R> {
    pub fn new(find_places: FindPlace<R>, get_forecast_info: GetForecastInfo<R>) -> Self {
        Self {
            find_places,
            get_forecast_info,
        }
    }
}

impl<R: ForecastRepository> FindAndForecastInfo<R> {
    pub async fn execute(
        &self,
        places: Vec<Place>,
        types: Vec<PlaceType>,
        from_time: Time,
        to_time: Time,
        parameters: Vec<Parameter>,
        lang: Lang,
    ) -> Result<Vec<ForecastInfo>> {
        let mut forecast_infos = Vec::new();

        for chunk in places.chunks(30) {
            let resolved: Vec<(Place, Option<Location>)> = join_all(chunk.iter().map(|place| {
                let path = Path::FindPlaces(place.clone(), types.clone(), lang.clone());
                async move {
                    let location = self.find_places.execute(path, place.clone()).await;
                    (place.clone(), location)
                }
            }))
            .await;

            let mut locations: Vec<Location> = Vec::new();
            let mut not_found_locations: Vec<Place> = Vec::new();

            for (place, location) in resolved {
                match location {
                    Some(l) => locations.push(l),
                    None => not_found_locations.push(place),
                }
            }

            if !locations.is_empty() {
                let get_forecast_info_path = Path::GetForecastInfo(
                    locations.clone(),
                    parameters.clone(),
                    from_time.clone(),
                    to_time.clone(),
                    lang.clone(),
                );

                if let Some(forecast_info) =
                    self.get_forecast_info.execute(get_forecast_info_path).await
                {
                    forecast_infos.push(forecast_info);
                } else {
                    let place_days_not_found = locations
                        .iter()
                        .map(|location| {
                            PlaceDays::new(
                                location.place.clone(),
                                Vec::new(),
                                PlaceDaysStatus::ForecastInfoNotFound,
                            )
                        })
                        .collect();
                    forecast_infos.push(ForecastInfo::new(place_days_not_found));
                }
            }

            if !not_found_locations.is_empty() {
                let place_days_not_found = not_found_locations
                    .iter()
                    .map(|place| {
                        PlaceDays::new(place.clone(), Vec::new(), PlaceDaysStatus::LocationNotFound)
                    })
                    .collect();
                forecast_infos.push(ForecastInfo::new(place_days_not_found));
            }
        }

        Ok(forecast_infos)
    }
}
