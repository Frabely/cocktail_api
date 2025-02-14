use sqlx::Type;
use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Type)]
#[sqlx(type_name = "language_code")]
pub enum LanguageCode {
    #[sqlx(rename = "en-US")]
    EnUS,
    #[sqlx(rename = "de-DE")]
    DeDE,
}

impl TryFrom<&str> for LanguageCode {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "en-US" => Ok(LanguageCode::EnUS),
            "de-DE" => Ok(LanguageCode::DeDE),
            _ => Err(format!("Invalid LanguageCode value: {}", value)),
        }
    }
}