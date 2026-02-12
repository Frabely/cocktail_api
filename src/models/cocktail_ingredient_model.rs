use chrono::NaiveDate;
use crate::models::measure_unit_model::MeasureUnitModel;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CocktailIngredientModel {
    pub id: i32,
    pub ingredient_id: i32,
    pub cocktail_id: i32,
    pub name: String,
    pub alcohol_volume: f64,
    pub quantity: f64,
    pub measure_unit: MeasureUnitModel,
    pub modified_on: NaiveDate,
    pub created_on: NaiveDate,
}