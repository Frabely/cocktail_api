use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub language: Option<i32>,
    pub unit_of_measure_liquids: Option<i32>,
    pub unit_of_measure_solids: Option<i32>,
}