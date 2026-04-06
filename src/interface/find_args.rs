use clap::{ArgAction, Parser};

use crate::{
    domain::{lang::Lang, location::PlaceType},
    interface::cli::PlacePair,
};

#[derive(Parser, Debug, Clone)]
#[command(about = "Find places")]
pub struct FindArgs {
    #[arg(short, long)]
    pub place: PlacePair,

    #[arg(short, long, num_args = 1.., action = ArgAction::Append, default_value = "locality,beach")]
    pub types: Vec<PlaceType>,

    #[arg(short, long)]
    pub lang: Option<Lang>,
}
