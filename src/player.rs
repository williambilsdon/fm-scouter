use fm_scouter_derive::ApplyWeights;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    #[serde(rename(deserialize = "Name"))]
    name: String,
    #[serde(rename(deserialize = "Inf"))]
    info: Option<String>,
    #[serde(rename(deserialize = "Position"))]
    position: String,
    #[serde(flatten)]
    attributes: Attributes,
}

impl Player {
    pub fn calculate_score(&mut self, weights: &Attributes) -> u64 {
        self.attributes.apply_weights(weights).into_iter().sum()
    }
}

#[derive(Debug, Deserialize, Serialize, ApplyWeights)]
pub struct Attributes {
    #[serde(rename(deserialize = "Wor"))]
    workrate: u64,
    #[serde(rename(deserialize = "Vis"))]
    vision: u64,
    #[serde(rename(deserialize = "Thr"))]
    throwing: u64,
    #[serde(rename(deserialize = "Tec"))]
    technique: u64,
    #[serde(rename(deserialize = "Tea"))]
    teamwork: u64,
    #[serde(rename(deserialize = "Tck"))]
    tackling: u64,
    #[serde(rename(deserialize = "Str"))]
    strength: u64,
    #[serde(rename(deserialize = "Sta"))]
    stamina: u64,
    #[serde(rename(deserialize = "TRO"))]
    rushing_out: u64,
    #[serde(rename(deserialize = "Ref"))]
    reflexes: u64,
    #[serde(rename(deserialize = "Pun"))]
    punching: u64,
    #[serde(rename(deserialize = "Pos"))]
    positioning: u64,
    #[serde(rename(deserialize = "Pen"))]
    penalties: u64,
    #[serde(rename(deserialize = "Pas"))]
    passing: u64,
    #[serde(rename(deserialize = "Pac"))]
    pace: u64,
    #[serde(rename(deserialize = "1v1"))]
    one_vs_one: u64,
    #[serde(rename(deserialize = "OtB"))]
    off_the_ball: u64,
    #[serde(rename(deserialize = "Nat"))]
    natural_fitness: u64,
    #[serde(rename(deserialize = "Mar"))]
    marking: u64,
    #[serde(rename(deserialize = "L Th"))]
    long_throws: u64,
    #[serde(rename(deserialize = "Lon"))]
    long_shots: u64,
    #[serde(rename(deserialize = "Ldr"))]
    leadership: u64,
    #[serde(rename(deserialize = "Kic"))]
    kicking: u64,
    #[serde(rename(deserialize = "Jum"))]
    jumping: u64,
    #[serde(rename(deserialize = "Hea"))]
    heading: u64,
    #[serde(rename(deserialize = "Han"))]
    handling: u64,
    #[serde(rename(deserialize = "Fre"))]
    free_kicks: u64,
    #[serde(rename(deserialize = "Fla"))]
    flair: u64,
    #[serde(rename(deserialize = "Fir"))]
    first_touch: u64,
    #[serde(rename(deserialize = "Fin"))]
    finishing: u64,
    #[serde(rename(deserialize = "Ecc"))]
    eccentricity: u64,
    #[serde(rename(deserialize = "Dri"))]
    dribbling: u64,
    #[serde(rename(deserialize = "Det"))]
    determination: u64,
    #[serde(rename(deserialize = "Dec"))]
    decision_making: u64,
    #[serde(rename(deserialize = "Cro"))]
    crossing: u64,
    #[serde(rename(deserialize = "Cor"))]
    corners: u64,
    #[serde(rename(deserialize = "Age"))]
    age: u64,
    #[serde(rename(deserialize = "Cnt"))]
    concentration: u64,
    #[serde(rename(deserialize = "Cmp"))]
    composure: u64,
    #[serde(rename(deserialize = "Com"))]
    communication: u64,
    #[serde(rename(deserialize = "Cmd"))]
    command_of_area: u64,
    #[serde(rename(deserialize = "Bra"))]
    bravery: u64,
    #[serde(rename(deserialize = "Bal"))]
    balance: u64,
    #[serde(rename(deserialize = "Ant"))]
    anticipation: u64,
    #[serde(rename(deserialize = "Agi"))]
    agility: u64,
    #[serde(rename(deserialize = "Agg"))]
    aggression: u64,
    #[serde(rename(deserialize = "Aer"))]
    aerial: u64,
    #[serde(rename(deserialize = "Acc"))]
    acceleration: u64,
}
