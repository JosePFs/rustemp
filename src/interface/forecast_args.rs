use clap::{ArgAction, Parser};

use crate::{
    domain::{lang::Lang, location::PlaceType, parameter::Parameter, time::Time},
    interface::cli::PlacePair,
};

#[derive(Parser, Debug)]
#[command(about = "Get forecast information")]
pub struct ForecastArgs {
    #[arg(short, long, required = true, num_args = 1.., action = ArgAction::Append, value_name = "place/municipality")]
    pub places: Vec<PlacePair>,

    #[arg(short, long, required = true)]
    pub start_time: Time,

    #[arg(short, long, required = true)]
    pub end_time: Time,

    #[arg(short, long, num_args = 1.., action = ArgAction::Append, default_value = "temperature,relative_humidity,precipitation_amount")]
    pub data: Vec<Parameter>,

    #[arg(short, long, num_args = 1.., action = ArgAction::Append, default_value = "locality,beach")]
    pub types: Vec<PlaceType>,

    #[arg(short, long)]
    pub lang: Option<Lang>,
}
