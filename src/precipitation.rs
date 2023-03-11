pub async fn get_rain(lat: f64, lon: f64) -> String {
    reqwest::get(format!("https://gpsgadget.buienradar.nl/data/raintext?lat={lat}&lon={lon}"))
        .await
        .expect("Failed to get a response from raintext API")
        .text()
        .await
        .expect("Failed to get payload from raintext API response")
}