use std::{
    any::Any,
    error::Error,
    fmt::{Debug, Display},
    mem::discriminant,
    num::ParseIntError,
};

use csv::StringRecord;

use crate::attribute::Attribute;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    info: Option<String>,
    position: String,
    age: u8,
    pub attributes: Vec<Attribute>,
    pub score: Option<u64>,
}

impl Player {
    pub fn calculate_score(&mut self, weights: &Vec<Attribute>) -> u64 {
        let score = self
            .attributes
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

        self.score = Some(score);
        score
    }

    pub fn from_string_record(
        player_record: &StringRecord,
        headers: &StringRecord,
    ) -> Result<Player, PlayerError> {
        let mut name = String::new();
        let mut info = None;
        let mut position = String::new();
        let mut age = 0;
        let mut attributes = Vec::new();
        for (idx, header) in headers.into_iter().enumerate() {
            let value = player_record.get(idx).ok_or(PlayerError::GetRecordValue)?;
            match header {
                "Name" => name = value.to_string(),
                "Inf" => info = Some(value.to_string()),
                "Position" => position = value.to_string(),
                "Age" => {
                    age = value
                        .parse::<u8>()
                        .map_err(|err| PlayerError::ParseIntError(err))?
                }
                _ => {
                    let value_u8: u8 = value
                        .parse()
                        .map_err(|err| PlayerError::ParseIntError(err))?;
                    attributes.push(Attribute::from_key_value(header, value_u8))
                }
            }
        }

        Ok(Player {
            name,
            info,
            position,
            age,
            attributes,
            score: None,
        })
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.score {
            Some(score_value) => writeln!(f, "Player: {}, Score: {}", self.name, score_value),
            None => writeln!(f, "Player: {} has no score calculated", self.name),
        }
    }
}

#[derive(Debug)]
pub enum PlayerError {
    GetRecordValue,
    ParseIntError(ParseIntError),
}

impl Display for PlayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerError::GetRecordValue => write!(f, "Failed to get record value"),
            PlayerError::ParseIntError(e) => write!(f, "Failed to parse int: {}", e.to_string()),
        }
    }
}

impl Error for PlayerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PlayerError::GetRecordValue => None,
            PlayerError::ParseIntError(e) => Some(e),
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

#[cfg(test)]
mod tests {
    use crate::{attribute::Attribute, player::Player};

    #[test]
    fn calculate_score_sums_correctly() {
        let mut player = Player {
            name: String::new(),
            info: None,
            position: String::new(),
            age: 0,
            attributes: vec![Attribute::Heading(10)],
            score: None,
        };

        let weights = vec![Attribute::Heading(10)];

        let result = player.calculate_score(&weights);
        assert_eq!(result, 100);
        assert_eq!(player.score, Some(100))
    }

    #[test]
    fn calculate_score_sums_mismatched_weight_correctly() {
        let mut player = Player {
            name: String::new(),
            info: None,
            position: String::new(),
            age: 0,
            attributes: vec![Attribute::Heading(10)],
            score: None,
        };

        let weights = vec![Attribute::Finishing(10)];

        let result = player.calculate_score(&weights);
        assert_eq!(result, 0);
        assert_eq!(player.score, Some(0))
    }

    #[test]
    fn calculate_score_sums_no_weights_correctly() {
        let mut player = Player {
            name: String::new(),
            info: None,
            position: String::new(),
            age: 0,
            attributes: vec![Attribute::Heading(10)],
            score: None,
        };

        let weights = Vec::new();

        let result = player.calculate_score(&weights);
        assert_eq!(result, 0);
        assert_eq!(player.score, Some(0))
    }
}
