use std::path::PathBuf;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[structopt(name = "buic", about = "the compact Rust buienradar JSON API CLI")]
pub struct Buic {
    /// Longitude
    #[clap(long, required = true)]
    longitude: f64,

    /// Latitude>
    #[clap(long, required = true)]
    latitude: f64,

    /// Output file, stdout if not present
    #[clap(short, long, requires = "filetype")]
    output: Option<PathBuf>,

    /// Output filetype
    #[clap(short, long, value_enum, ignore_case = true)]
    filetype: Option<FileType>,
}


#[derive(ValueEnum, Debug, Clone)]
enum FileType {
    CSV,
    JSON
}
