use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::models::cocktail_ingredient_model::CocktailIngredientModel;
use serde_json::Value;

#[derive(Serialize, Deserialize, FromRow)]
pub struct CocktailModel {
    pub id: i32,
    pub name: String,
    pub is_alcoholic: bool,
    pub modified_on: NaiveDate,
    pub created_on: NaiveDate,
    pub ingredients: Vec<CocktailIngredientModel>
}

pub struct CocktailRow {
    pub id: i32,
    pub name: String,
    pub is_alcoholic: bool,
    pub modified_on: NaiveDate,
    pub created_on: NaiveDate,
    pub ingredients: Value,
}


