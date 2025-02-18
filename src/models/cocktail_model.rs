use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::models::ingredient_model::IngredientModel;

#[derive(Serialize, Deserialize, FromRow)]
pub struct CocktailModel {
    id: i32,
    name: String,
    is_alcoholic: bool,
    modified_on: chrono::NaiveDateTime,
    created_on: chrono::NaiveDateTime,
    ingredients: Vec<IngredientModel>
}