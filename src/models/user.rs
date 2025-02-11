use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::common::enums::language_code::LanguageCode;
use crate::common::enums::unit_of_measure_liquids::UnitOfMeasureLiquids;
use crate::common::enums::unit_of_measure_solids::UnitOfMeasureSolids;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub language_code: LanguageCode,
    pub unit_of_measure_liquids: UnitOfMeasureLiquids,
    pub unit_of_measure_solids: UnitOfMeasureSolids,
}
