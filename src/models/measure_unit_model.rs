use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::common::enums::measure_unit::MeasureUnit;

#[derive(Serialize, Deserialize, FromRow)]
pub struct MeasureUnitModel {
    id: i32,
    name: String,
    abbreviation: MeasureUnit,
}