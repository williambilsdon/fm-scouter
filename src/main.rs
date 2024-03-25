use std::fs::File;

use clap::Parser;
use fm_scouter::parse_csv;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    current_squad_file_path: String,
}

fn main() {
    let args = Args::parse();

    let current_squad_file = File::open(args.current_squad_file_path).unwrap();

    // let weights_file = File::open("./weights/advanced_forward.json").unwrap();
    // let weights: Vec<Attributes> = serde_json::from_reader(weights_file).unwrap();

    let players = parse_csv(current_squad_file).unwrap();

    for player in players {
        println!("Name: {:?}", player)
    }
}
