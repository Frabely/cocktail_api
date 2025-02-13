use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum UnitOfMeasureLiquids {
    Litres = 1,
    Millilitres = 2,
}

impl TryFrom<i32> for UnitOfMeasureLiquids {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(UnitOfMeasureLiquids::Litres),
            2 => Ok(UnitOfMeasureLiquids::Millilitres),
            _ => Err(format!("Invalid UnitOfMeasureLiquids value: {}", value)),
        }
    }
}