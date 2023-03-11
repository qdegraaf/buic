pub async fn get_rain(lat: f64, lon: f64) -> Result<(), reqwest::Error> {
    let resp = reqwest::get(format!("https://gpsgadget.buienradar.nl/data/raintext?lat={lat}&lon={lon}"))
        .await?.text().await?;
    println!("{}", resp);

    Ok(())
}