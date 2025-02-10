use sqlx::PgPool;
use crate::models::user::User;

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT id, name, email, created_at FROM users")
        .fetch_all(pool)
        .await
}