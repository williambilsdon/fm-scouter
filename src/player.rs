use std::fmt::{Debug, Display};

use crate::attribute::Attributes;

pub mod parser;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    info: Option<String>,
    position: String,
    age: u8,
    pub attributes: Attributes,
    pub score: u64,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Player: {}, Score: {}", self.name, self.score)
    }
}
