use std::{
    error::Error,
    fmt::{Debug, Display},
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
}

impl Player {
    pub fn calculate_score(&mut self, weights: &Vec<Attribute>) -> u64 {
        unimplemented!()
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
        })
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
