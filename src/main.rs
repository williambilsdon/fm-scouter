use std::{
    error::Error,
    fs::{read_to_string, File},
    path::Path,
};

use clap::Parser;
use fm_scouter::{parse_csv, parse_weights};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    current_squad_file_path: String,
    scouted_players_file_path: String,
}

fn main() {
    let args = Args::parse();

    if let Err(err) = app(args) {
        println!("{}", err);
        std::process::exit(1)
    } else {
        std::process::exit(0)
    }
}

fn app(args: Args) -> Result<(), Box<dyn Error>> {
    let weights_string = read_to_string(Path::new("./weights/advanced_forward.json"))?;
    let weights = parse_weights(weights_string.as_str())?;

    println!("Prased Weights");

    let current_squad_file = File::open(args.current_squad_file_path)?;

    let current_players = parse_csv(current_squad_file, &weights)?;

    println!("Parsed Players");

    for player in current_players.iter() {
        println!("{}", player)
    }

    let scouted_players_file = File::open(args.scouted_players_file_path)?;

    let scouted_players = parse_csv(scouted_players_file, &weights)?;

    for player in scouted_players.iter() {
        println!("{}", player)
    }

    Ok(())
}
