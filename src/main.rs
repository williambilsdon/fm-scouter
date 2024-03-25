use std::{error::Error, fs::File};

use clap::Parser;
use csv::StringRecord;
use fm_scouter::player::{Player, PlayerError};

fn parse_csv(csv_file: File) -> Result<Vec<Player>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(csv_file);
    let headers = { rdr.headers()?.clone() };

    let result_records: Vec<StringRecord> = rdr
        .records()
        .collect::<Result<Vec<StringRecord>, csv::Error>>()?;

    let players = result_records
        .iter()
        .map(|record| Player::from_string_record(record, &headers))
        .collect::<Result<Vec<Player>, PlayerError>>()?;

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

    // let weights_file = File::open("./weights/advanced_forward.json").unwrap();
    // let weights: Vec<Attributes> = serde_json::from_reader(weights_file).unwrap();

    let players = parse_csv(current_squad_file).unwrap();

    for player in players {
        println!("Name: {:?}", player)
    }
}
