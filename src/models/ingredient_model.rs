use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct IngredientModel {
    id: i32,
    name: String,
    alcohol_volume: f64,
    modified_on: chrono::NaiveDateTime,
    created_on: chrono::NaiveDateTime,
}