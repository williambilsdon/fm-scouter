use crate::player::{Player, PlayerError};
use attribute::{Attribute, Weights};
use csv::StringRecord;
use std::{error::Error, fs::File};

mod attribute;
pub mod player;

pub fn parse_csv(csv_file: File) -> Result<Vec<Player>, Box<dyn Error>> {
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

pub fn parse_weights(weight_file: &str) -> Result<Vec<Attribute>, Box<dyn Error>> {
    let weights: Weights = serde_json::from_str(weight_file)?;
    Ok(weights.weights)
}
