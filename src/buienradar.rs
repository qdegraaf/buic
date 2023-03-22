use anyhow::anyhow;
use serde::Deserialize;
use std::fmt;
use std::fmt::Display;

pub async fn get_rain(lat: f64, lon: f64) -> String {
    reqwest::get(format!(
        "https://gpsgadget.buienradar.nl/data/raintext?lat={lat}&lon={lon}"
    ))
    .await
    .expect("Failed to get a response from raintext API")
    .text()
    .await
    .expect("Failed to get payload from raintext API response")
}

pub async fn get_actuals(station: String) -> Result<StationMeasurement, anyhow::Error> {
    let response = reqwest::get("https://data.buienradar.nl/2.0/feed/json")
        .await
        .expect("Failed to get a response from JSON API")
        .text()
        .await
        .expect("Failed to get payload from JSON API response");

    let json: JSONResult =
        serde_json::from_str(&response).expect("Could not read JSON into JSONResult value");

    for station_measurement in json.actual.station_measurements {
        if station_measurement
            .station_name
            .to_lowercase()
            .contains(&station)
        {
            return Ok(station_measurement);
        }
    }
    Err(anyhow!("No measurement found for station: {}", station))
}

#[derive(Deserialize, Debug)]
struct JSONResult {
    #[serde(alias = "$id")]
    _id: String,
    actual: Actual,
    forecast: Forecast,
}

#[derive(Deserialize, Debug)]
struct Actual {
    #[serde(alias = "$id")]
    _id: String,
    #[serde(alias = "actualradarurl")]
    _actual_radar_url: String,
    _sunrise: String,
    _sunset: String,
    #[serde(alias = "stationmeasurements")]
    station_measurements: Vec<StationMeasurement>,
}

#[derive(Deserialize, Debug)]
pub struct StationMeasurement {
    #[serde(alias = "$id")]
    id: String,
    #[serde(alias = "stationid")]
    station_id: u32,
    #[serde(alias = "stationname")]
    station_name: String,
    lat: f32,
    lon: f32,
    regio: String,
    timestamp: String,
    #[serde(alias = "weatherdescription")]
    weather_description: String,
    #[serde(alias = "graphUrl")]
    graph_url: String,
    #[serde(alias = "winddirection")]
    wind_direction: Option<String>,
    temperature: Option<f32>,
    #[serde(alias = "groundtemperature")]
    ground_temperature: Option<f32>,
    #[serde(alias = "feeltemperature")]
    feel_temperature: Option<f32>,
    #[serde(alias = "windgusts")]
    wind_gusts: Option<f32>,
    #[serde(alias = "windspeed")]
    wind_speed: Option<f32>,
    #[serde(alias = "windspeedBft")]
    wind_speed_bft: Option<f32>,
    humidity: Option<f32>,
    #[serde(alias = "precipitation")]
    precipitation: Option<f32>,
    #[serde(alias = "sunpower")]
    sun_power: Option<f32>,
    #[serde(alias = "rainFallLast24Hour")]
    rainfall_last_24_hour: Option<f32>,
    #[serde(alias = "rainFallLastHour")]
    rainfall_last_hour: Option<f32>,
    #[serde(alias = "winddirectiondegrees")]
    wind_direction_degrees: Option<f32>,
}

impl Display for StationMeasurement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Actual:
           ID: {}
           Station ID: {}
           Station Name: {}
           Latitude: {}
           Longitude: {}
           Regio: {}
           Timestamp: {}
           Weather Description: {}
           Graph URL: {}\n",
            self.id,
            self.station_id,
            self.station_name,
            self.lat,
            self.lon,
            self.regio,
            self.timestamp,
            self.weather_description,
            self.graph_url,
        )?;
        write_optional_field::<String>(String::from("Wind Direction"), &self.wind_direction, f)?;
        write_optional_field::<f32>(String::from("Temperature"), &self.temperature, f)?;
        write_optional_field::<f32>(
            String::from("Ground Temperature"),
            &self.ground_temperature,
            f,
        )?;
        write_optional_field::<f32>(String::from("Feel Temperature"), &self.feel_temperature, f)?;
        write_optional_field::<f32>(String::from("Wind Gusts"), &self.wind_gusts, f)?;
        write_optional_field::<f32>(String::from("Wind Speed"), &self.wind_speed, f)?;
        write_optional_field::<f32>(String::from("Wind Speed Bft"), &self.wind_speed_bft, f)?;
        write_optional_field::<f32>(String::from("Humidity"), &self.humidity, f)?;
        write_optional_field::<f32>(String::from("Precipitation"), &self.precipitation, f)?;
        write_optional_field::<f32>(String::from("Sunpower"), &self.sun_power, f)?;
        write_optional_field::<f32>(
            String::from("Rainfall Last 24 Hours"),
            &self.rainfall_last_24_hour,
            f,
        )?;
        write_optional_field::<f32>(
            String::from("Rainfall Last Hour"),
            &self.rainfall_last_hour,
            f,
        )?;
        write_optional_field::<f32>(
            String::from("Wind Direction Degrees"),
            &self.wind_direction_degrees,
            f,
        )
    }
}

fn write_optional_field<T: Display>(
    key: String,
    val: &Option<T>,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    match val {
        Some(value) => writeln!(f, "           {}: {}", key, value),
        None => Ok(()),
    }
}

#[derive(Deserialize, Debug)]
struct Forecast {
    #[serde(alias = "$id")]
    _id: String,
    #[serde(alias = "fivedayforecast")]
    five_day_forecast: Vec<DayForecast>,
}

#[derive(Deserialize, Debug)]
pub struct DayForecast {
    #[serde(alias = "$id")]
    id: String,
    day: String,
    #[serde(alias = "mintemperature")]
    min_temperature: String,
    #[serde(alias = "maxtemperature")]
    max_temperature: String,
    #[serde(alias = "mintemperatureMax")]
    min_temperature_max: i32,
    #[serde(alias = "mintemperatureMin")]
    min_temperature_min: i32,
    #[serde(alias = "maxtemperatureMax")]
    max_temperature_max: i32,
    #[serde(alias = "maxtemperatureMin")]
    max_temperature_min: i32,
    #[serde(alias = "rainChance")]
    rain_chance: f32,
    #[serde(alias = "sunChance")]
    sun_chance: f32,
    #[serde(alias = "windDirection")]
    wind_direction: String,
    wind: f32,
    #[serde(alias = "mmRainMin")]
    mm_rain_min: f32,
    #[serde(alias = "mmRainMax")]
    mm_rain_max: f32,
    #[serde(alias = "weatherdescription")]
    weather_description: String,
}

impl Display for DayForecast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Forecast:
           ID: {}
           Day: {}
           Min Temperature: {}
           Max Temperature: {}
           Min Temperature Max: {}
           Min Temperature Min: {}
           Max Temperature Max: {}
           Max Temperature Min: {}
           Rain Chance: {}
           Sun Chance: {}
           Wind Direction: {}
           Wind: {}
           mm Rain Min: {}
           mm Rain Max: {}
           Weather Description: {}",
            self.id,
            self.day,
            self.min_temperature,
            self.max_temperature,
            self.min_temperature_max,
            self.min_temperature_min,
            self.max_temperature_max,
            self.max_temperature_min,
            self.rain_chance,
            self.sun_chance,
            self.wind_direction,
            self.wind,
            self.mm_rain_min,
            self.mm_rain_max,
            self.weather_description
        )
    }
}

pub async fn get_forecast(n_days: u8) -> Vec<DayForecast> {
    let response = reqwest::get("https://data.buienradar.nl/2.0/feed/json")
        .await
        .expect("Failed to get a response from JSON API")
        .text()
        .await
        .expect("Failed to get payload from JSON API response");

    let json: JSONResult =
        serde_json::from_str(&response).expect("Could not read JSON into JSONResult value");

    let n_days_us = usize::try_from(n_days).unwrap();
    json.forecast
        .five_day_forecast
        .into_iter()
        .take(n_days_us)
        .collect()
}
