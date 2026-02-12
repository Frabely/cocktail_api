use sqlx::{PgPool};
use std::env;

pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL muss gesetzt sein");
    PgPool::connect(&database_url).await
}
