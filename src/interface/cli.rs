use clap::Parser;

use std::str::FromStr;

use crate::{
    domain::{
        location::{Name, Place},
        municipality::Municipality,
    },
    interface::find_args::FindArgs,
    interface::forecast_args::ForecastArgs,
};

#[derive(Debug, Clone)]
pub struct PlacePair {
    pub location: String,
    pub municipality: String,
}

impl FromStr for PlacePair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, '/');

        Ok(PlacePair {
            location: parts.next().ok_or("missing location")?.to_string(),
            municipality: parts.next().ok_or("missing municipality")?.to_string(),
        })
    }
}

impl From<PlacePair> for Place {
    fn from(value: PlacePair) -> Self {
        Place::new(
            Name::from(value.location),
            Municipality::from(value.municipality),
        )
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    Forecast(ForecastArgs),
    Find(FindArgs),
}
