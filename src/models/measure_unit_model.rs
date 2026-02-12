use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::common::enums::measure_unit::MeasureUnit;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct MeasureUnitModel {
    pub id: i32,
    pub name: String,
    pub abbreviation: MeasureUnit,
}