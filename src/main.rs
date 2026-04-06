use clap::Parser as _;

use rustemp::{
    domain::error::Result,
    interface::{
        cli::{Args, Commands},
        to_json::{find_and_forecast_to_json, find_location_to_json},
    },
    run_find, run_find_and_forecast,
    tracing::init_logging,
};

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();

    let args = Args::parse();

    let json = match args.command {
        Commands::Forecast(forecast_args) => {
            find_and_forecast_to_json(run_find_and_forecast(forecast_args).await?)?
        }
        Commands::Find(find_args) => {
            find_location_to_json(run_find(find_args.clone()).await, find_args.place)?
        }
    };

    println!("{json}");

    Ok(())
}
