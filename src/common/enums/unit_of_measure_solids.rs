use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum UnitOfMeasureSolids {
    Gram = 1,
    Kilogram = 2,
    Milligram = 3,
}

impl TryFrom<i32> for UnitOfMeasureSolids {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(UnitOfMeasureSolids::Gram),
            2 => Ok(UnitOfMeasureSolids::Kilogram),
            3 => Ok(UnitOfMeasureSolids::Milligram),
            _ => Err(format!("Invalid UnitOfMeasureLiquids value: {}", value)),
        }
    }
}