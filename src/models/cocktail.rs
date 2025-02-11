use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Cocktail {
    id: i32,
    is_alcoholic: bool,
    modified_on: chrono::NaiveDateTime,
    created_on: chrono::NaiveDateTime,
}