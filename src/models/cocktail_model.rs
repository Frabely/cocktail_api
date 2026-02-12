use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::models::cocktail_ingredient_model::CocktailIngredientModel;

#[derive(Serialize, Deserialize, FromRow)]
pub struct CocktailModel {
    pub id: i32,
    pub name: String,
    pub is_alcoholic: bool,
    pub modified_on: chrono::NaiveDateTime,
    pub created_on: chrono::NaiveDateTime,
    pub ingredients: Vec<CocktailIngredientModel>
}

#[derive(FromRow, Debug)]
pub struct CocktailRow {
    pub id: i32,
    pub name: String,
    pub is_alcoholic: bool,
    pub modified_on: chrono::NaiveDateTime,
    pub created_on: chrono::NaiveDateTime,
    pub ingredients: String,
}

