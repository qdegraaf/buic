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
    rainChane: u16,
    sunChance: u16,
    windDirection: String,
    wind: u16,
    mmRainMin: u32,
    mmRainMax: u32,
    weatherdescription: String,
    iconurl: String,
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