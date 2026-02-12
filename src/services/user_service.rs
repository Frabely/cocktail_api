/*use sqlx::{Error, PgPool};
use crate::common::enums::language_code::LanguageCode;
use crate::common::enums::unit_of_measure_liquids::UnitOfMeasureLiquids;
use crate::common::enums::unit_of_measure_solids::UnitOfMeasureSolids;
use crate::models::user_model::UserModel;
use crate::common::dtos::create_user_dto::CreateUserDto;

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<UserModel>, Error> {
    let users = sqlx::query!(
        r#"
        SELECT
            id,
            username AS name,
            email,
            created_on,
            language AS "language: LanguageCode",
            unit_of_measure_liquids,
            unit_of_measure_solids
        FROM general.users
        "#
    )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| {
            let unit_of_measure_liquids = row
                .unit_of_measure_liquids
                .map(UnitOfMeasureLiquids::try_from)
                .transpose()
                .map_err(|e| Error::Decode(e.into()))?;

            let unit_of_measure_solids = row
                .unit_of_measure_solids
                .map(UnitOfMeasureSolids::try_from)
                .transpose()
                .map_err(|e| Error::Decode(e.into()))?;

            Ok(UserModel {
                id: row.id,
                name: row.name,
                email: row.email,
                created_on: row.created_on,
                language: row.language,
                unit_of_measure_liquids,
                unit_of_measure_solids,
            })
        })
        .collect::<Result<Vec<UserModel>, Error>>()?;

    Ok(users)
}



// pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, Error> {
//     let result = sqlx::query!(
//         r#"
//         SELECT id, username, email, created_on, language, unit_of_measure_liquids, unit_of_measure_solids
//         FROM general.users
//         "#
//     )
//         .fetch_all(pool)
//         .await?;
//
//     let users = result
//         .into_iter()
//         .map(|row| {
//             let language = match row.language {
//                 Some(lang) => match LanguageCode::try_from(lang) {
//                     Ok(parsed_lang) => Some(parsed_lang),
//                     Err(_) => return Err(Error::from(Error::Decode("Invalid language code".into()))),
//                 },
//                 None => None,
//             };
//
//             let unit_of_measure_liquids = match row.unit_of_measure_liquids {
//                 Some(unit) => match UnitOfMeasureLiquids::try_from(unit) {
//                     Ok(parsed_unit) => Some(parsed_unit),
//                     Err(_) => return Err(Error::from(Error::Decode("Invalid unit_of_measure_liquids code".into()))),
//                 },
//                 None => None,
//             };
//
//             let unit_of_measure_solids = match row.unit_of_measure_solids {
//                 Some(unit) => match UnitOfMeasureSolids::try_from(unit) {
//                     Ok(parsed_unit) => Some(parsed_unit),
//                     Err(_) => return Err(Error::from(Error::Decode("Invalid unit_of_measure_solids code".into()))),
//                 },
//                 None => None,
//             };
//
//             Ok(User {
//                 id: row.id,
//                 name: row.username,
//                 email: row.email,
//                 created_on: row.created_on,
//                 language,
//                 unit_of_measure_liquids,
//                 unit_of_measure_solids,
//             })
//         })
//         .collect::<Result<Vec<User>, Error>>()?;
//
//     Ok(users)
// }

pub async fn create_user(pool: &PgPool, user_input: CreateUserDto) -> Result<UserModel, Error> {
    let lang = user_input.language.as_deref();
    let result = sqlx::query!(
        r#"
        INSERT INTO general.users (username, email, language, unit_of_measure_liquids, unit_of_measure_solids)
        VALUES ($1, $2, $3::text::general.language_code, $4, $5)
        RETURNING id, created_on, username, email,
                  language AS "language: Option<LanguageCode>",
                  unit_of_measure_liquids,
                  unit_of_measure_solids
        "#,
        user_input.username,
        user_input.email,
        lang,
        user_input.unit_of_measure_liquids,
        user_input.unit_of_measure_solids
    )
        .fetch_one(pool)
        .await?;

    Ok(UserModel {
        id: result.id,
        name: result.username,
        email: result.email,
        created_on: result.created_on,
        language: result.language.unwrap(),
        unit_of_measure_liquids: Some(UnitOfMeasureLiquids::try_from(result.unit_of_measure_liquids.unwrap()).unwrap()),
        unit_of_measure_solids: Some(UnitOfMeasureSolids::try_from(result.unit_of_measure_solids.unwrap()).unwrap()),
    })
}


*/