use attribute::{Attribute, WeightWrapper, Weights};
use csv::StringRecord;
use player::parser::{parse, ParserError};
use player::Player;
use std::thread::{self, ScopedJoinHandle};
use std::{error::Error, fs::File};

pub mod attribute;
mod player;

pub fn parse_csv(csv_file: File, weights: &Weights) -> Result<Vec<Player>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(csv_file);
    let headers = { rdr.headers()?.clone() };

    let string_records: Vec<StringRecord> = rdr
        .records()
        .collect::<Result<Vec<StringRecord>, csv::Error>>()?;

    let players = par_parse_records(&string_records, &headers, weights)?;

    let filtered_players = players
        .into_iter()
        .filter(|player| player.name != "- -")
        .collect();

    Ok(filtered_players)
}

pub fn parse_records(
    string_records: &[StringRecord],
    headers: &StringRecord,
    weights: &Weights,
) -> Result<Vec<Player>, ParserError> {
    string_records
        .iter()
        .map(|record| parse(record, &headers, weights))
        .collect()
}

pub fn par_parse_records(
    string_records: &[StringRecord],
    headers: &StringRecord,
    weights: &Weights,
) -> Result<Vec<Player>, ParserError> {
    let chunk_size = string_records.len() / 4;
    let chunks = string_records.chunks(chunk_size);
    thread::scope(|s| {
        let thread_results: Vec<ScopedJoinHandle<'_, Result<Vec<Player>, ParserError>>> = chunks
            .map(move |chunk| s.spawn(|| parse_records(chunk, headers, weights)))
            .collect();

        thread_results
            .into_iter()
            .map(|thread| thread.join().unwrap())
            .try_fold(Vec::new(), |mut acc, result| {
                acc.extend(result?);
                Ok(acc)
            })
    })
}

pub fn parse_weights(weight_file: &str) -> Result<Vec<Attribute>, Box<dyn Error>> {
    let weights: WeightWrapper = serde_json::from_str(weight_file)?;
    Ok(weights.weights)
}
