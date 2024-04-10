use std::{
    error::Error,
    fs::{read_to_string, File},
    path::Path,
};

use clap::Parser;
use fm_scouter::{
    parse_csv, parse_weights,
    player::{self, Player},
};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    current_squad_file_path: String,
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
    let current_squad_file = File::open(args.current_squad_file_path)?;

    let mut players = parse_csv(current_squad_file)?;

    println!("Parsed Players");

    let weights_string = read_to_string(Path::new("./weights/advanced_forward.json"))?;
    let weights = parse_weights(weights_string.as_str())?;

    println!("Parsed Weights");

    for player in players.iter_mut() {
        let _ = player.calculate_score(&weights);
        println!("{}", player)
    }

    Ok(())
}
