use sqlx::{Error, PgPool};
use crate::common::enums::language_code::LanguageCode;
use crate::common::enums::unit_of_measure_liquids::UnitOfMeasureLiquids;
use crate::common::enums::unit_of_measure_solids::UnitOfMeasureSolids;
use crate::models::user::User;
use crate::models::create_user_dto::CreateUserDto;

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, Error> {
    let result = sqlx::query!(
        r#"
        SELECT id, username, email, created_on, language, unit_of_measure_liquids, unit_of_measure_solids
        FROM general.users
        "#
    )
        .fetch_all(pool)
        .await?;

    let users = result
        .into_iter()
        .map(|row| {
            let language = match row.language {
                Some(lang) => match LanguageCode::try_from(lang) {
                    Ok(parsed_lang) => Some(parsed_lang),
                    Err(_) => return Err(Error::from(sqlx::Error::Decode("Invalid language code".into()))),
                },
                None => None,
            };

            let unit_of_measure_liquids = match row.unit_of_measure_liquids {
                Some(unit) => match UnitOfMeasureLiquids::try_from(unit) {
                    Ok(parsed_unit) => Some(parsed_unit),
                    Err(_) => return Err(Error::from(sqlx::Error::Decode("Invalid unit_of_measure_liquids code".into()))),
                },
                None => None,
            };

            let unit_of_measure_solids = match row.unit_of_measure_solids {
                Some(unit) => match UnitOfMeasureSolids::try_from(unit) {
                    Ok(parsed_unit) => Some(parsed_unit),
                    Err(_) => return Err(Error::from(sqlx::Error::Decode("Invalid unit_of_measure_solids code".into()))),
                },
                None => None,
            };

            Ok(User {
                id: row.id,
                name: row.username,
                email: row.email,
                created_on: row.created_on,
                language,
                unit_of_measure_liquids,
                unit_of_measure_solids,
            })
        })
        .collect::<Result<Vec<User>, Error>>()?;

    Ok(users)
}

struct UserOutput {
    id: i32,
    created_on: chrono::NaiveDateTime,
}

pub async fn create_user(pool: &PgPool, user_input: CreateUserDto) -> Result<User, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        INSERT INTO general.users (username, email, language, unit_of_measure_liquids, unit_of_measure_solids)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, created_on, language, unit_of_measure_liquids, unit_of_measure_solids
        "#,
        user_input.username,
        user_input.email,
        user_input.language,
        user_input.unit_of_measure_liquids,
        user_input.unit_of_measure_solids
    )
        .fetch_one(pool)
        .await?;

    let language = match result.language {
        Some(lang) => match LanguageCode::try_from(lang) {
            Ok(parsed_lang) => Some(parsed_lang),
            Err(_) => return Err(sqlx::Error::Decode("Invalid language code".into())),
        },
        None => None,
    };

    let unit_of_measure_liquids = match result.unit_of_measure_liquids {
        Some(unit) => match UnitOfMeasureLiquids::try_from(unit) {
            Ok(parsed_unit) => Some(parsed_unit),
            Err(_) => return Err(sqlx::Error::Decode("Invalid unit_of_measure_liquids code".into())),
        },
        None => None,
    };

    let unit_of_measure_solids = match result.unit_of_measure_solids {
        Some(unit) => match UnitOfMeasureSolids::try_from(unit) {
            Ok(parsed_unit) => Some(parsed_unit),
            Err(_) => return Err(sqlx::Error::Decode("Invalid unit_of_measure_solids code".into())),
        },
        None => None,
    };

    let user = User {
        id: result.id,
        name: user_input.username.clone(),
        email: user_input.email.clone(),
        created_on: result.created_on,
        language,
        unit_of_measure_liquids,
        unit_of_measure_solids,
    };

    Ok(user)
}

