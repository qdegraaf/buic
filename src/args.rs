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
    Actuals {
        #[clap(short, long, required = true)]
        station: String,
    },
    Forecast {
        #[clap(short, long, required = true)]
        n_days: u8,
    },
}
