use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct IngredientModel {
    pub id: i32,
    pub name: String,
    pub alcohol_volume: f64,
    pub modified_on: chrono::NaiveDateTime,
    pub created_on: chrono::NaiveDateTime,
}