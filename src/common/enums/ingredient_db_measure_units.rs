use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum IngredientDbMeasureUnits {
    None = 0,
    Millilitres = 1,
    Gram = 2,
    Piece = 3
}

impl TryFrom<i32> for IngredientDbMeasureUnits {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(IngredientDbMeasureUnits::None),
            1 => Ok(IngredientDbMeasureUnits::Millilitres),
            2 => Ok(IngredientDbMeasureUnits::Gram),
            3 => Ok(IngredientDbMeasureUnits::Piece),
            _ => Err(format!("Invalid IngredientDbMeasures value: {}", value)),
        }
    }
}