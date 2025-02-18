use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::models::measure_unit_model::MeasureUnitModel;

#[derive(Serialize, Deserialize, FromRow)]
pub struct CocktailIngredientModel {
    id: i32,
    name: String,
    alcohol_volume: f64,
    quantity: f64,
    measure_unit: MeasureUnitModel,
    modified_on: chrono::NaiveDateTime,
    created_on: chrono::NaiveDateTime,
}