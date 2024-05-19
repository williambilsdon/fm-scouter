use std::{
    collections::HashMap, error::Error, fmt::Display, mem::discriminant, num::ParseIntError,
};

use csv::StringRecord;

use crate::attribute::{match_headers, Attribute, Attributes, Weights};

use super::Player;

pub fn parse(
    player_record: &StringRecord,
    headers: &StringRecord,
    weights: &Weights,
) -> Result<Player, ParserError> {
    let headers_idx: HashMap<&str, usize> = headers
        .into_iter()
        .enumerate()
        .map(|(idx, header)| (header, idx))
        .collect();

    let headers_str_values = headers_idx
        .into_iter()
        .filter_map(|(header, idx)| {
            let str_value = player_record.get(idx);
            match str_value {
                Some(value) => Some((header, value)),
                None => None,
            }
        })
        .collect::<HashMap<&str, &str>>();

    let name = headers_str_values
        .get("Name")
        .ok_or_else(|| ParserError::HeaderIndexError("Name".to_string()))?
        .to_string();

    let info = Some(
        headers_str_values
            .get("Inf")
            .ok_or_else(|| ParserError::HeaderIndexError("Inf".to_string()))?
            .to_string(),
    );

    let position = headers_str_values
        .get("Position")
        .ok_or_else(|| ParserError::HeaderIndexError("Position".to_string()))?
        .to_string();

    let age = headers_str_values
        .get("Age")
        .ok_or_else(|| ParserError::HeaderIndexError("Age".to_string()))?
        .parse::<u8>()
        .map_err(|err| ParserError::ParseIntError(err))?;

    let attributes: Attributes = headers_str_values
        .into_iter()
        .filter(|(header, _)| match_headers(header))
        .filter_map(|(header, value)| {
            let parsed_value = value
                .parse::<u8>()
                .map_err(|err| ParserError::ParseIntError(err)) // TODO: This result map is useless...
                // While it simplifies our types I'd like to preserve this error as at this point we know the header
                // should contain an attribute value and if it doesn't we don't want to parse the player at all
                .ok();

            match parsed_value {
                Some(value) => Some((header, value)),
                None => None,
            }
        })
        .map(|(header, value)| Attribute::from_key_value(header, value))
        .collect();

    let score = calculate_score(&attributes, weights);
    let player = Player {
        name,
        info,
        position,
        age,
        attributes,
        score,
    };
    println!("Created Player: {:?}", player);
    Ok(player)
}

fn calculate_score(attributes: &Attributes, weights: &Weights) -> u64 {
    let score = attributes
        .iter()
        .map(|attr| {
            let attr_val: u64 = attr.get_value().to_owned().into();
            let weight_val: u64 = weights.iter().fold(0, |_, weight| {
                if discriminant(attr) == discriminant(weight) {
                    weight.get_value().to_owned().into()
                } else {
                    0
                }
            });
            attr_val * weight_val
        })
        .sum();

    score
}

#[derive(Debug)]
pub enum ParserError {
    HeaderIndexError(String),
    IdxNotFound(usize, String),
    ParseIntError(ParseIntError),
}

impl Error for ParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::HeaderIndexError(header) => {
                write!(f, "Failed to get index for header: {}", header)
            }
            ParserError::IdxNotFound(idx, header) => write!(
                f,
                "Failed to find value at index: {} for header: {}",
                idx, header
            ),
            ParserError::ParseIntError(err) => write!(f, "Failed to parse Int with err: {}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        attribute::Attribute,
        player::{parser::calculate_score, Player},
    };

    #[test]
    fn calculate_score_sums_correctly() {
        let player = Player {
            name: String::new(),
            info: None,
            position: String::new(),
            age: 0,
            attributes: vec![Attribute::Heading(10)],
            score: 0,
        };

        let weights = vec![Attribute::Heading(10)];

        let result = calculate_score(&player.attributes, &weights);
        assert_eq!(result, 100);
    }

    #[test]
    fn calculate_score_sums_mismatched_weight_correctly() {
        let player = Player {
            name: String::new(),
            info: None,
            position: String::new(),
            age: 0,
            attributes: vec![Attribute::Heading(10)],
            score: 0,
        };

        let weights = vec![Attribute::Finishing(10)];

        let result = calculate_score(&player.attributes, &weights);
        assert_eq!(result, 0);
    }

    #[test]
    fn calculate_score_sums_no_weights_correctly() {
        let player = Player {
            name: String::new(),
            info: None,
            position: String::new(),
            age: 0,
            attributes: vec![Attribute::Heading(10)],
            score: 0,
        };

        let weights = Vec::new();

        let result = calculate_score(&player.attributes, &weights);
        assert_eq!(result, 0);
    }
}
