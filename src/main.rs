use std::fs::File;
use std::io::Write;

use buic::args::{Buic, BuicCommand, WeatherCommand};
use buic::buienradar::{get_actuals, get_forecast, get_rain};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Buic::parse();

    match args.cmd {
        BuicCommand::Rain {
            latitude,
            longitude,
        } => {
            let response = get_rain(latitude, longitude).await;
            write_output(&args, response);
        }
        BuicCommand::Weather { ref cmd } => match cmd {
            WeatherCommand::Actuals { station } => {
                let response = get_actuals(station.to_string()).await?;
                write_output(&args, response.to_string());
            }
            WeatherCommand::Forecast { n_days } => {
                let response = get_forecast(*n_days).await;
                match args.output {
                    Some(path) => {
                        let mut file =
                            File::create(path).expect("Could not create file for given path");
                        for forecast in response {
                            file.write_all(forecast.to_string().as_ref())
                                .expect("Could not write response to file");
                        }
                    }
                    None => {
                        for forecast in response {
                            println!("{}", forecast)
                        }
                    }
                };
            }
        },
    };

    Ok(())
}

fn write_output(args: &Buic, response: String) {
    match &args.output {
        Some(path) => {
            let mut file = File::create(path).expect("Could not create file for given path");
            file.write_all(response.as_ref())
                .expect("Could not write response to file");
        }
        None => println!("{}", response),
    };
}
