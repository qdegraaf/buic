use buic::args::Buic;
use clap::Parser;

fn main() {
    let args = Buic::parse();
    println!("{:?}", args);
}
