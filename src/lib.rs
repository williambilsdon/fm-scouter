use crate::player::{Player, PlayerError};
use attribute::{Attribute, WeightWrapper, Weights};
use csv::StringRecord;
use std::{error::Error, fs::File};

mod attribute;
pub mod player;

pub fn parse_csv(csv_file: File, weights: &Weights) -> Result<Vec<Player>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(csv_file);
    let headers = { rdr.headers()?.clone() };

    let result_records: Vec<StringRecord> = rdr
        .records()
        .collect::<Result<Vec<StringRecord>, csv::Error>>()?;

    let players = result_records
        .iter()
        .map(|record| Player::from_string_record(record, &headers, &weights))
        .collect::<Result<Vec<Player>, PlayerError>>()?;

    let filtered_players = players
        .into_iter()
        .filter(|player| !player.name.is_empty())
        .collect();

    Ok(filtered_players)
}

pub fn parse_weights(weight_file: &str) -> Result<Vec<Attribute>, Box<dyn Error>> {
    let weights: WeightWrapper = serde_json::from_str(weight_file)?;
    Ok(weights.weights)
}
