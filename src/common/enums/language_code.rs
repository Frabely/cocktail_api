use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum LanguageCode {
    EnUS = 1,
    DeDE = 2
}

impl TryFrom<i32> for LanguageCode {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(LanguageCode::EnUS),
            2 => Ok(LanguageCode::DeDE),
            _ => Err(format!("Invalid LanguageCode value: {}", value)),
        }
    }
}