use attribute::{Attribute, WeightWrapper, Weights};
use csv::StringRecord;
use player::parser::{parse, ParserError};
use player::Player;
use std::{error::Error, fs::File};

mod attribute;
mod player;

pub fn parse_csv(csv_file: File, weights: &Weights) -> Result<Vec<Player>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(csv_file);
    let headers = { rdr.headers()?.clone() };

    let result_records: Vec<StringRecord> = rdr
        .records()
        .collect::<Result<Vec<StringRecord>, csv::Error>>()?;

    let players = result_records
        .iter()
        .map(|record| parse(record, &headers, weights))
        .collect::<Result<Vec<Player>, ParserError>>()?;

    let filtered_players = players
        .into_iter()
        .filter(|player| player.name != "- -")
        .collect();

    Ok(filtered_players)
}

pub fn parse_weights(weight_file: &str) -> Result<Vec<Attribute>, Box<dyn Error>> {
    let weights: WeightWrapper = serde_json::from_str(weight_file)?;
    Ok(weights.weights)
}
