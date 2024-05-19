use serde::{de::Visitor, Deserialize, Deserializer};

pub type Attributes = Vec<Attribute>;
pub type Weights = Vec<Attribute>;

#[derive(Debug, Deserialize)]
pub enum Attribute {
    Workrate(u8),
    Vision(u8),
    Throwing(u8),
    Technique(u8),
    Teamwork(u8),
    Tackling(u8),
    Strength(u8),
    Stamina(u8),
    RushingOut(u8),
    Reflexes(u8),
    Punching(u8),
    Positioning(u8),
    Penalties(u8),
    Passing(u8),
    Pace(u8),
    OneVsOne(u8),
    OffTheBall(u8),
    NaturalFitness(u8),
    Marking(u8),
    LongThrows(u8),
    LongShots(u8),
    Leadership(u8),
    Kicking(u8),
    Jumping(u8),
    Heading(u8),
    Handling(u8),
    FreeKicks(u8),
    Flair(u8),
    FirstTouch(u8),
    Finishing(u8),
    Eccentricity(u8),
    Dribbling(u8),
    Determination(u8),
    DecisionMaking(u8),
    Crossing(u8),
    Corners(u8),
    Concentration(u8),
    Composure(u8),
    Communication(u8),
    CommandOfArea(u8),
    Bravery(u8),
    Balance(u8),
    Anticipation(u8),
    Agility(u8),
    Aggression(u8),
    Aerial(u8),
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
    pub fn get_value(&self) -> &u8 {
        match self {
            Attribute::Workrate(v) => v,
            Attribute::Vision(v) => v,
            Attribute::Throwing(v) => v,
            Attribute::Technique(v) => v,
            Attribute::Teamwork(v) => v,
            Attribute::Tackling(v) => v,
            Attribute::Strength(v) => v,
            Attribute::Stamina(v) => v,
            Attribute::RushingOut(v) => v,
            Attribute::Reflexes(v) => v,
            Attribute::Punching(v) => v,
            Attribute::Positioning(v) => v,
            Attribute::Penalties(v) => v,
            Attribute::Passing(v) => v,
            Attribute::Pace(v) => v,
            Attribute::OneVsOne(v) => v,
            Attribute::OffTheBall(v) => v,
            Attribute::NaturalFitness(v) => v,
            Attribute::Marking(v) => v,
            Attribute::LongThrows(v) => v,
            Attribute::LongShots(v) => v,
            Attribute::Leadership(v) => v,
            Attribute::Kicking(v) => v,
            Attribute::Jumping(v) => v,
            Attribute::Heading(v) => v,
            Attribute::Handling(v) => v,
            Attribute::FreeKicks(v) => v,
            Attribute::Flair(v) => v,
            Attribute::FirstTouch(v) => v,
            Attribute::Finishing(v) => v,
            Attribute::Eccentricity(v) => v,
            Attribute::Dribbling(v) => v,
            Attribute::Determination(v) => v,
            Attribute::DecisionMaking(v) => v,
            Attribute::Crossing(v) => v,
            Attribute::Corners(v) => v,
            Attribute::Concentration(v) => v,
            Attribute::Composure(v) => v,
            Attribute::Communication(v) => v,
            Attribute::CommandOfArea(v) => v,
            Attribute::Bravery(v) => v,
            Attribute::Balance(v) => v,
            Attribute::Anticipation(v) => v,
            Attribute::Agility(v) => v,
            Attribute::Aggression(v) => v,
            Attribute::Aerial(v) => v,
            Attribute::Acceleration(v) => v,
        }
    }
}

pub fn match_headers(key: &str) -> bool {
    match key {
        "Wor" => true,
        "Vis" => true,
        "Thr" => true,
        "Tec" => true,
        "Tea" => true,
        "Tck" => true,
        "Str" => true,
        "Sta" => true,
        "TRO" => true,
        "Ref" => true,
        "Pun" => true,
        "Pos" => true,
        "Pen" => true,
        "Pas" => true,
        "Pac" => true,
        "1v1" => true,
        "OtB" => true,
        "Nat" => true,
        "Mar" => true,
        "L Th" => true,
        "Lon" => true,
        "Ldr" => true,
        "Kic" => true,
        "Jum" => true,
        "Hea" => true,
        "Han" => true,
        "Fre" => true,
        "Fla" => true,
        "Fir" => true,
        "Fin" => true,
        "Ecc" => true,
        "Dri" => true,
        "Det" => true,
        "Dec" => true,
        "Cro" => true,
        "Cor" => true,
        "Cnt" => true,
        "Cmp" => true,
        "Com" => true,
        "Cmd" => true,
        "Bra" => true,
        "Bal" => true,
        "Ant" => true,
        "Agi" => true,
        "Agg" => true,
        "Aer" => true,
        "Acc" => true,
        _ => false,
    }
}
// TODO: This is an annoying wrapper struct I seem to need in order to deserialise the weights properly
// Would be great to spend more time to get rid of this but for now will just carry on.
#[derive(Debug, Deserialize)]
pub struct WeightWrapper {
    #[serde(deserialize_with = "deserialise_attr_vec")]
    pub weights: Vec<Attribute>,
}

pub fn deserialise_attr_vec<'de, D>(deserializer: D) -> Result<Vec<Attribute>, D::Error>
where
    D: Deserializer<'de>,
{
    struct AttributeVisitor;

    impl<'de> Visitor<'de> for AttributeVisitor {
        type Value = Vec<Attribute>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("expecting map of key values pairs where the key is an attribute header and value is attribute value as u8")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let mut result = vec![];
            while let Some((key, value)) = map.next_entry()? {
                result.push(self::Attribute::from_key_value(key, value))
            }

            Ok(result)
        }
    }

    deserializer.deserialize_map(AttributeVisitor)
}
