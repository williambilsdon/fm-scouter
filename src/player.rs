use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
struct Attributes {
    #[serde(rename(deserialize = "Wor"))]
    workrate: u8,
    #[serde(rename(deserialize = "Vis"))]
    vision: u8,
    #[serde(rename(deserialize = "Thr"))]
    throwing: u8,
    #[serde(rename(deserialize = "Tec"))]
    technique: u8,
    #[serde(rename(deserialize = "Tea"))]
    teamwork: u8,
    #[serde(rename(deserialize = "Tck"))]
    tackling: u8,
    #[serde(rename(deserialize = "Str"))]
    strength: u8,
    #[serde(rename(deserialize = "Sta"))]
    stamina: u8,
    #[serde(rename(deserialize = "TRO"))]
    rushing_out: u8,
    #[serde(rename(deserialize = "Ref"))]
    reflexes: u8,
    #[serde(rename(deserialize = "Pun"))]
    punching: u8,
    #[serde(rename(deserialize = "Pos"))]
    positioning: u8,
    #[serde(rename(deserialize = "Pen"))]
    penalties: u8,
    #[serde(rename(deserialize = "Pas"))]
    passing: u8,
    #[serde(rename(deserialize = "Pac"))]
    pace: u8,
    #[serde(rename(deserialize = "1v1"))]
    one_vs_one: u8,
    #[serde(rename(deserialize = "OtB"))]
    off_the_ball: u8,
    #[serde(rename(deserialize = "Nat"))]
    natural_fitness: u8,
    #[serde(rename(deserialize = "Mar"))]
    marking: u8,
    #[serde(rename(deserialize = "L Th"))]
    long_throws: u8,
    #[serde(rename(deserialize = "Lon"))]
    long_shots: u8,
    #[serde(rename(deserialize = "Ldr"))]
    leadership: u8,
    #[serde(rename(deserialize = "Kic"))]
    kicking: u8,
    #[serde(rename(deserialize = "Jum"))]
    jumping: u8,
    #[serde(rename(deserialize = "Hea"))]
    heading: u8,
    #[serde(rename(deserialize = "Han"))]
    handling: u8,
    #[serde(rename(deserialize = "Fre"))]
    free_kicks: u8,
    #[serde(rename(deserialize = "Fla"))]
    flair: u8,
    #[serde(rename(deserialize = "Fir"))]
    first_touch: u8,
    #[serde(rename(deserialize = "Fin"))]
    finishing: u8,
    #[serde(rename(deserialize = "Ecc"))]
    eccentricity: u8,
    #[serde(rename(deserialize = "Dri"))]
    dribbling: u8,
    #[serde(rename(deserialize = "Det"))]
    determination: u8,
    #[serde(rename(deserialize = "Dec"))]
    decision_making: u8,
    #[serde(rename(deserialize = "Cro"))]
    crossing: u8,
    #[serde(rename(deserialize = "Cor"))]
    corners: u8,
    #[serde(rename(deserialize = "Age"))]
    age: u8,
    #[serde(rename(deserialize = "Cnt"))]
    concentration: u8,
    #[serde(rename(deserialize = "Cmp"))]
    composure: u8,
    #[serde(rename(deserialize = "Com"))]
    communication: u8,
    #[serde(rename(deserialize = "Cmd"))]
    command_of_area: u8,
    #[serde(rename(deserialize = "Bra"))]
    bravery: u8,
    #[serde(rename(deserialize = "Bal"))]
    balance: u8,
    #[serde(rename(deserialize = "Ant"))]
    anticipation: u8,
    #[serde(rename(deserialize = "Agi"))]
    agility: u8,
    #[serde(rename(deserialize = "Agg"))]
    aggression: u8,
    #[serde(rename(deserialize = "Aer"))]
    aerial: u8,
    #[serde(rename(deserialize = "Acc"))]
    acceleration: u8,
}
