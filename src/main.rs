use buic::args::Buic;
use buic::precipitation::get_rain;
use clap::Parser;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let args = Buic::parse();

    get_rain(args.latitude, args.longitude).await?;
    println!("{:?}", args);

    Ok(())
}
