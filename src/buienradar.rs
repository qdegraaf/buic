use std::fmt;

use serde::Deserialize;

pub async fn get_rain(lat: f64, lon: f64) -> String {
    reqwest::get(format!("https://gpsgadget.buienradar.nl/data/raintext?lat={lat}&lon={lon}"))
        .await
        .expect("Failed to get a response from raintext API")
        .text()
        .await
        .expect("Failed to get payload from raintext API response")
}

pub async fn get_actuals(station: String) -> String {
    let response = reqwest::get("https://data.buienradar.nl/2.0/feed/json")
        .await
        .expect("Failed to get a response from JSON API")
        .text()
        .await
        .expect("Failed to get payload from JSON API response");

    let json: JSONResult = serde_json::from_str(&response)
        .expect("Could not read JSON into JSONResult value");

    for station_measurement in json.actual.station_measurements {
        if station_measurement.station_name.to_lowercase().contains(&station) {
            let result = station_measurement;
            println!("{:?}", result);
            break;
        }
    }
    String::from("OKASDASD")
}

#[derive(Deserialize, Debug)]
struct JSONResult {
    #[serde(alias = "$id")]
    id: String,
    actual: Actual,
    forecast: Forecast,
}

#[derive(Deserialize, Debug)]
struct Actual {
    #[serde(alias = "$id")]
    id: String,
    #[serde(alias = "actualradarurl")]
    actual_radar_url: String,
    sunrise: String,
    sunset: String,
    #[serde(alias = "stationmeasurements")]
    station_measurements: Vec<StationMeasurement>,
}

#[derive(Deserialize, Debug)]
struct StationMeasurement {
    #[serde(alias = "$id")]
    id: String,
    #[serde(alias = "stationname")]
    station_name: String,
}

#[derive(Deserialize, Debug)]
struct Forecast {
    #[serde(alias = "$id")]
    id: String,
    #[serde(alias = "fivedayforecast")]
    five_day_forecast: Vec<DayForecast>,
}

#[derive(Deserialize, Debug)]
pub struct DayForecast {
    #[serde(alias = "$id")]
    id: String,
    day: String,
    mintemperature: String,
    maxtemperature: String,
    mintemperatureMax: i32,
    mintemperatureMin: i32,
    maxtemperatureMax: i32,
    maxtemperatureMin: i32,
    rainChance: u16,
    sunChance: u16,
    windDirection: String,
    wind: u16,
    mmRainMin: f32,
    mmRainMax: f32,
    weatherdescription: String,
    iconurl: String,
}

impl fmt::Display for DayForecast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Forecast:\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
               self.id, self.day, self.mintemperature, self.maxtemperature, self.mintemperatureMax,
               self.mintemperatureMin, self.maxtemperatureMax, self.maxtemperatureMin,
               self.rainChance, self.sunChance, self.windDirection, self.wind, self.mmRainMin,
               self.mmRainMax, self.weatherdescription, self.iconurl)
    }
}

pub async fn get_forecast(n_days: u8) -> Vec<DayForecast> {
    let response = reqwest::get("https://data.buienradar.nl/2.0/feed/json")
        .await
        .expect("Failed to get a response from JSON API")
        .text()
        .await
        .expect("Failed to get payload from JSON API response");

    let json: JSONResult = serde_json::from_str(&response)
        .expect("Could not read JSON into JSONResult value");

    let n_days_us = usize::try_from(n_days).unwrap();
    json.forecast.five_day_forecast.into_iter().take(n_days_us).collect()
}