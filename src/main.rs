use std::fs::File;
use std::io::Write;

use clap::Parser;

use buic::args::{Buic, BuicCommand, WeatherCommand};
use buic::buienradar::{get_actuals, get_forecast, get_rain};


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Buic::parse();

    match args.cmd {
        BuicCommand::Rain { latitude, longitude } => {
            let response = get_rain(latitude, longitude).await;
            match args.output {
                Some(path) => {
                    let mut file = File::create(path).expect("Could not create file for given path");
                    file.write_all(response.as_ref()).expect("Could not write response to file");
                }
                None => println!("{}", response)
            };
        },
        BuicCommand::Weather { cmd } => match cmd {
            WeatherCommand::Actuals { station } => {
                let response = get_actuals(station).await;
                match args.output {
                    Some(path) => {
                        let mut file = File::create(path).expect("Could not create file for given path");
                        file.write_all(response.as_ref()).expect("Could not write response to file");
                    }
                    None => println!("{}", response)
                };
            },
            WeatherCommand::Forecast { n_days } => {
                let response = get_forecast(n_days).await;
                println!("{:?}", response);
            }
        }
    };

    Ok(())
}

// // TODO: Add better error handling
// // TODO: Add multiple file type handling
// fn write_output(args: &Buic, response: String) {
//     match args.output {
//         Some(path) => {
//             let mut file = File::create(path).expect("Could not create file for given path");
//             file.write_all(response.as_ref()).expect("Could not write response to file");
//         }
//         None => println!("{}", response)
//     };
// }