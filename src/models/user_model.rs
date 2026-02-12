// use serde::{Serialize, Deserialize};
// use sqlx::{FromRow, Row};
// use crate::common::enums::{
//     language_code::LanguageCode,
//     unit_of_measure_liquids::UnitOfMeasureLiquids,
//     unit_of_measure_solids::UnitOfMeasureSolids};
//
// #[derive(Serialize, Deserialize, FromRow)]
// pub struct UserModel {
//     pub id: i32,
//     pub name: String,
//     pub email: String,
//     pub created_on: chrono::NaiveDateTime,
//     pub language: Option<LanguageCode>,
//     pub unit_of_measure_liquids: Option<UnitOfMeasureLiquids>,
//     pub unit_of_measure_solids: Option<UnitOfMeasureSolids>,
// }
//
