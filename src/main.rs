use std::{fs::File, process};

use clap::Parser;
use fm_scouter::player::Player;

fn parse_csv(current_squad_file: File) -> Result<(), csv::Error> {
    let mut rdr = csv::Reader::from_reader(current_squad_file);
    let players: Vec<Player> = rdr
        .deserialize()
        .map(|result: Result<Player, csv::Error>| {
            result.expect("failed to deserialise players into player structs")
        })
        .collect::<Vec<Player>>();
    println!("{:?}", players);
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    current_squad_file_path: String,
}

fn main() {
    let args = Args::parse();

    let current_squad_file = File::open(args.current_squad_file_path).unwrap();

    if let Err(err) = parse_csv(current_squad_file) {
        println!("error parsing current squad csv: {}", err);
        process::exit(1);
    }
}
