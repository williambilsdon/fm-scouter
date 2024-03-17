use std::{fs::File, process};

use clap::Parser;
use fm_scouter::player::{Attributes, Player};

fn parse_csv(current_squad_file: File, weights: Attributes) -> Result<Vec<Player>, csv::Error> {
    let mut rdr = csv::Reader::from_reader(current_squad_file);

    let mut players = rdr
        .deserialize()
        .map(|result: Result<Player, csv::Error>| {
            result.expect("failed to deserialise players into player structs")
        })
        .collect::<Vec<Player>>();

    for player in &mut players {
        println!(
            "{}: {:?}",
            player.name.clone(),
            player.calculate_score(&weights)
        )
    }

    Ok(players)
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    current_squad_file_path: String,
}

fn main() {
    let args = Args::parse();

    let current_squad_file = File::open(args.current_squad_file_path).unwrap();

    let weights_file = File::open("./weights/advanced_forward.json").unwrap();
    let weights: Attributes = serde_json::from_reader(weights_file).unwrap();

    if let Err(err) = parse_csv(current_squad_file, weights) {
        println!("error parsing current squad csv: {}", err);
        process::exit(1);
    }
}
