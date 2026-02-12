use sqlx::{PgPool};
use crate::models::cocktail_ingredient_model::CocktailIngredientModel;
use crate::models::cocktail_model::CocktailModel;
use crate::models::cocktail_model::CocktailRow;

pub async fn get_all_cocktails(pool: &PgPool) -> Result<Vec<CocktailModel>, sqlx::Error> {
    let query = sqlx::query_as!(
        CocktailRow,
        r#"
        SELECT
            c.id as "id!",
            ct.name as "name!",
            COALESCE(c.is_alcoholic, false) as "is_alcoholic!",
            c.modified_on as "modified_on!",
            c.created_on as "created_on!",
            COALESCE(
                json_agg(
                    CASE WHEN ci.id IS NOT NULL THEN
                        json_build_object(
                            'id', ci.id,
                            'ingredient_id', i.id,
                            'cocktail_id', ci.cocktail_id,
                            'name', i_t.name,
                            'alcohol_volume', i.alcohol_volume,
                            'quantity', ci.quantity,
                            'measure_unit', json_build_object(
                                'id', mu.id,
                                'name', mu_t.name,
                                'abbreviation', mu.abbreviation::text
                            ),
                            'modified_on', ci.modified_on,
                            'created_on', ci.created_on
                        )
                    END
                ) FILTER (WHERE ci.id IS NOT NULL),
                '[]'
            )::json::text as "ingredients!"
        FROM drinks.cocktails c
        LEFT JOIN drinks.cocktail_translation ct ON c.id = ct.cocktail_id
        LEFT JOIN drinks.cocktail_ingredients ci ON c.id = ci.cocktail_id
        LEFT JOIN drinks.ingredients i ON ci.ingredients_id = i.id
        LEFT JOIN drinks.ingredient_translations i_t ON i.id = i_t.ingredient_id AND i_t.language_code::text = $1
        LEFT JOIN general.measure_units mu ON ci.measure_unit_id = mu.id
        LEFT JOIN general.measure_unit_translations mu_t ON mu.id = mu_t.measure_unit_id AND mu_t.language_code::text = $1
        WHERE ct.language_code::text = $1
        GROUP BY c.id, ct.name, c.is_alcoholic, c.modified_on, c.created_on
        "#,
        "en-US"
    )
        .fetch_all(pool)
        .await?;

    let result = query
        .into_iter()
        .map(|row| {
            let ingredients: Vec<CocktailIngredientModel> = serde_json::from_str(&row.ingredients)
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

            Ok(CocktailModel {
                id: row.id,
                name: row.name,
                is_alcoholic: row.is_alcoholic,
                modified_on: row.modified_on,
                created_on: row.created_on,
                ingredients,
            })
        })
        .collect();
    result
}