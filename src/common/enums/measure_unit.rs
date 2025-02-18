use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeasureUnit {
    None = 0,
    Ml = 1,
    Drop = 2,
    Tsp = 3,
    Tbsp = 4,
    Dash = 5,
    Splash = 6,
    G = 7,
    Pcs = 8,
    Leaf = 9,
    Clove = 10,
    Stick = 11,
    Pinch = 12,
    Kt = 13,
    Bunch = 14,
}

impl TryFrom<i32> for MeasureUnit {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MeasureUnit::Ml),
            2 => Ok(MeasureUnit::Drop),
            3 => Ok(MeasureUnit::Tsp),
            4 => Ok(MeasureUnit::Tbsp),
            5 => Ok(MeasureUnit::Dash),
            6 => Ok(MeasureUnit::Splash),
            7 => Ok(MeasureUnit::G),
            8 => Ok(MeasureUnit::Pcs),
            9 => Ok(MeasureUnit::Leaf),
            10 => Ok(MeasureUnit::Clove),
            11 => Ok(MeasureUnit::Stick),
            12 => Ok(MeasureUnit::Pinch),
            13 => Ok(MeasureUnit::Kt),
            14 => Ok(MeasureUnit::Bunch),
            _ => Err(format!("Invalid MeasureUnit value: {}", value)),
        }
    }
}