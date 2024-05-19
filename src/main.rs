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

    let current_squad_file = File::open(args.current_squad_file_path)?;

    let current_players = parse_csv(current_squad_file, &weights)?;

    let scouted_players_file = File::open(args.scouted_players_file_path)?;

    let scouted_players = parse_csv(scouted_players_file, &weights)?;

    let mut all_players: Vec<_> = current_players
        .iter()
        .chain(scouted_players.iter())
        .collect();

    all_players.sort_by(|a, b| b.score.cmp(&a.score));

    for player in all_players {
        println!("{}", player)
    }
    Ok(())
}
