use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Attribute {
    #[serde(rename(deserialize = "Wor"))]
    Workrate(u8),
    #[serde(rename(deserialize = "Vis"))]
    Vision(u8),
    #[serde(rename(deserialize = "Thr"))]
    Throwing(u8),
    #[serde(rename(deserialize = "Tec"))]
    Technique(u8),
    #[serde(rename(deserialize = "Tea"))]
    Teamwork(u8),
    #[serde(rename(deserialize = "Tck"))]
    Tackling(u8),
    #[serde(rename(deserialize = "Str"))]
    Strength(u8),
    #[serde(rename(deserialize = "Sta"))]
    Stamina(u8),
    #[serde(rename(deserialize = "TRO"))]
    RushingOut(u8),
    #[serde(rename(deserialize = "Ref"))]
    Reflexes(u8),
    #[serde(rename(deserialize = "Pun"))]
    Punching(u8),
    #[serde(rename(deserialize = "Pos"))]
    Positioning(u8),
    #[serde(rename(deserialize = "Pen"))]
    Penalties(u8),
    #[serde(rename(deserialize = "Pas"))]
    Passing(u8),
    #[serde(rename(deserialize = "Pac"))]
    Pace(u8),
    #[serde(rename(deserialize = "1v1"))]
    OneVsOne(u8),
    #[serde(rename(deserialize = "OtB"))]
    OffTheBall(u8),
    #[serde(rename(deserialize = "Nat"))]
    NaturalFitness(u8),
    #[serde(rename(deserialize = "Mar"))]
    Marking(u8),
    #[serde(rename(deserialize = "L Th"))]
    LongThrows(u8),
    #[serde(rename(deserialize = "Lon"))]
    LongShots(u8),
    #[serde(rename(deserialize = "Ldr"))]
    Leadership(u8),
    #[serde(rename(deserialize = "Kic"))]
    Kicking(u8),
    #[serde(rename(deserialize = "Jum"))]
    Jumping(u8),
    #[serde(rename(deserialize = "Hea"))]
    Heading(u8),
    #[serde(rename(deserialize = "Han"))]
    Handling(u8),
    #[serde(rename(deserialize = "Fre"))]
    FreeKicks(u8),
    #[serde(rename(deserialize = "Fla"))]
    Flair(u8),
    #[serde(rename(deserialize = "Fir"))]
    FirstTouch(u8),
    #[serde(rename(deserialize = "Fin"))]
    Finishing(u8),
    #[serde(rename(deserialize = "Ecc"))]
    Eccentricity(u8),
    #[serde(rename(deserialize = "Dri"))]
    Dribbling(u8),
    #[serde(rename(deserialize = "Det"))]
    Determination(u8),
    #[serde(rename(deserialize = "Dec"))]
    DecisionMaking(u8),
    #[serde(rename(deserialize = "Cro"))]
    Crossing(u8),
    #[serde(rename(deserialize = "Cor"))]
    Corners(u8),
    #[serde(rename(deserialize = "Cnt"))]
    Concentration(u8),
    #[serde(rename(deserialize = "Cmp"))]
    Composure(u8),
    #[serde(rename(deserialize = "Com"))]
    Communication(u8),
    #[serde(rename(deserialize = "Cmd"))]
    CommandOfArea(u8),
    #[serde(rename(deserialize = "Bra"))]
    Bravery(u8),
    #[serde(rename(deserialize = "Bal"))]
    Balance(u8),
    #[serde(rename(deserialize = "Ant"))]
    Anticipation(u8),
    #[serde(rename(deserialize = "Agi"))]
    Agility(u8),
    #[serde(rename(deserialize = "Agg"))]
    Aggression(u8),
    #[serde(rename(deserialize = "Aer"))]
    Aerial(u8),
    #[serde(rename(deserialize = "Acc"))]
    Acceleration(u8),
}

impl Attribute {
    pub fn from_key_value(key: &str, value: u8) -> Attribute {
        match key {
            "Wor" => Attribute::Workrate(value),
            "Vis" => Attribute::Vision(value),
            "Thr" => Attribute::Throwing(value),
            "Tec" => Attribute::Technique(value),
            "Tea" => Attribute::Teamwork(value),
            "Tck" => Attribute::Tackling(value),
            "Str" => Attribute::Strength(value),
            "Sta" => Attribute::Stamina(value),
            "TRO" => Attribute::RushingOut(value),
            "Ref" => Attribute::Reflexes(value),
            "Pun" => Attribute::Punching(value),
            "Pos" => Attribute::Positioning(value),
            "Pen" => Attribute::Penalties(value),
            "Pas" => Attribute::Passing(value),
            "Pac" => Attribute::Pace(value),
            "1v1" => Attribute::OneVsOne(value),
            "OtB" => Attribute::OffTheBall(value),
            "Nat" => Attribute::NaturalFitness(value),
            "Mar" => Attribute::Marking(value),
            "L Th" => Attribute::LongThrows(value),
            "Lon" => Attribute::LongShots(value),
            "Ldr" => Attribute::Leadership(value),
            "Kic" => Attribute::Kicking(value),
            "Jum" => Attribute::Jumping(value),
            "Hea" => Attribute::Heading(value),
            "Han" => Attribute::Handling(value),
            "Fre" => Attribute::FreeKicks(value),
            "Fla" => Attribute::Flair(value),
            "Fir" => Attribute::FirstTouch(value),
            "Fin" => Attribute::Finishing(value),
            "Ecc" => Attribute::Eccentricity(value),
            "Dri" => Attribute::Dribbling(value),
            "Det" => Attribute::Determination(value),
            "Dec" => Attribute::DecisionMaking(value),
            "Cro" => Attribute::Crossing(value),
            "Cor" => Attribute::Corners(value),
            "Cnt" => Attribute::Concentration(value),
            "Cmp" => Attribute::Composure(value),
            "Com" => Attribute::Communication(value),
            "Cmd" => Attribute::CommandOfArea(value),
            "Bra" => Attribute::Bravery(value),
            "Bal" => Attribute::Balance(value),
            "Ant" => Attribute::Anticipation(value),
            "Agi" => Attribute::Agility(value),
            "Agg" => Attribute::Aggression(value),
            "Aer" => Attribute::Aerial(value),
            "Acc" => Attribute::Acceleration(value),
            _ => panic!("Key here should match: {}", key),
        }
    }
}
