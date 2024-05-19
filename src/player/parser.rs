use std::{
    collections::HashMap, error::Error, fmt::Display, mem::discriminant, num::ParseIntError, vec,
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
        .map(|(header, value)| {
            let parsed_value: Result<u8, ParserError> = match value.contains("-") {
                true => parse_masked_attribute(value),
                false => value
                    .parse::<u8>()
                    .map_err(|err| ParserError::ParseIntError(err)),
            };

            match parsed_value {
                Ok(value) => Ok((header, value)),
                Err(err) => Err(err),
            }
        })
        .map(|result| match result {
            Ok((header, value)) => Ok(Attribute::from_key_value(header, value)),
            Err(err) => Err(err),
        })
        .collect::<Result<_, ParserError>>()?;

    let score = calculate_score(&attributes, weights);
    let player = Player {
        name,
        info,
        position,
        age,
        attributes,
        score,
    };
    Ok(player)
}

fn parse_masked_attribute(value: &str) -> Result<u8, ParserError> {
    if value == "-" {
        return Ok(0);
    }

    let split_value: Vec<&str> = value.split("-").collect();
    let parsed_values: Result<Vec<u8>, ParserError> = split_value
        .into_iter()
        .map(|val_str| {
            val_str
                .parse::<u8>()
                .map_err(|err| ParserError::ParseIntError(err))
        })
        .collect();

    match parsed_values {
        Ok(values) => Ok(values.into_iter().sum()),
        Err(err) => Err(err),
    }
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
