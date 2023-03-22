use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[structopt(name = "buic", about = "the compact Rust buienradar JSON API CLI")]
pub struct Buic {
    /// Type of Buic command to run
    #[clap(subcommand)]
    pub cmd: BuicCommand,

    /// Output file, stdout if not present
    #[clap(short, long)]
    pub output: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum BuicCommand {
    /// Get rain data 2h into the future for any lat/lon pair in NL/BE. Defaults to Utrecht
    Rain {
        /// Latitude>
        #[clap(long, default_value = "52.0907")]
        latitude: f64,

        /// Longitude
        #[clap(long, default_value = "5.1214")]
        longitude: f64,
    },
    /// Get more detailed weather info from the buienradar.nl JSON API
    Weather {
        /// Type of weather command to run
        #[clap(subcommand)]
        cmd: WeatherCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum WeatherCommand {
    /// Get actual weather data from weather stations around the Benelux
    Actuals {
        /// Name of the weather station for which you want to get actual weather data
        #[clap(short, long, required = true)]
        station: String,
    },
    /// Get the country wide forecast for 1-5 days into the future
    Forecast {
        /// Number of days into the future to get forecast for
        #[clap(short, long, required = true)]
        n_days: u8,
    },
}
