use std::{
    error::Error,
    fmt::{Debug, Display},
    mem::discriminant,
    num::ParseIntError,
};

use csv::StringRecord;

use crate::attribute::{Attribute, Attributes, Weights};

#[derive(Debug)]
pub struct Player {
    pub name: String,
    info: Option<String>,
    position: String,
    age: u8,
    pub attributes: Attributes,
    pub score: u64,
}

impl Player {
    pub fn from_string_record(
        player_record: &StringRecord,
        headers: &StringRecord,
        weights: &Vec<Attribute>,
    ) -> Result<Player, PlayerError> {
        let mut name = String::new();
        let mut info = None;
        let mut position = String::new();
        let mut age = 0;
        let mut attributes: Attributes = Vec::new();
        for (idx, header) in headers.into_iter().enumerate() {
            let value = player_record.get(idx).ok_or(PlayerError::GetRecordValue)?;
            if value == "- -" {
                break;
            }
            match header {
                "Name" => name = value.to_string(),
                "Inf" => info = Some(value.to_string()),
                "Position" => position = value.to_string(),
                "Age" => {
                    age = value
                        .parse::<u8>()
                        .map_err(|err| PlayerError::ParseIntError(err, value.to_string()))?
                }
                "Transfer Value" => continue,
                "Club" => continue,
                "Rec" => continue,
                _ => {
                    // We assume that all other fields are attributes, any fields that aren't attributes must be handled above.
                    let value_u8: u8 = {
                        if value.contains("-") {
                            let split_range = value.split("-").collect::<Vec<&str>>();
                            let results: Vec<u8> = split_range
                                .into_iter()
                                .map(|value| {
                                    if value.is_empty() {
                                        Ok(0)
                                    } else {
                                        value.parse::<u8>().map_err(|err| {
                                            PlayerError::ParseIntError(err, value.to_string())
                                        })
                                    }
                                })
                                .collect::<Result<_, PlayerError>>()?;

                            results.iter().sum()
                        } else {
                            value
                                .parse()
                                .map_err(|err| PlayerError::ParseIntError(err, value.to_string()))?
                        }
                    };

                    attributes.push(Attribute::from_key_value(header, value_u8))
                }
            }
        }

        let score = calculate_score(&attributes, weights);

        Ok(Player {
            name,
            info,
            position,
            age,
            attributes,
            score: score,
        })
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

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Player: {}, Score: {}", self.name, self.score)
    }
}

#[derive(Debug)]
pub enum PlayerError {
    GetRecordValue,
    ParseIntError(ParseIntError, String),
}

impl Display for PlayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerError::GetRecordValue => write!(f, "Failed to get record value"),
            PlayerError::ParseIntError(e, s) => {
                write!(
                    f,
                    "Failed to parse int: {}, With error: {}",
                    s,
                    e.to_string()
                )
            }
        }
    }
}

impl Error for PlayerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PlayerError::GetRecordValue => None,
            PlayerError::ParseIntError(e, _) => Some(e),
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

#[cfg(test)]
mod tests {
    use crate::{attribute::Attribute, player::calculate_score, player::Player};

    #[test]
    fn calculate_score_sums_correctly() {
        let mut player = Player {
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
        let mut player = Player {
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
        let mut player = Player {
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
