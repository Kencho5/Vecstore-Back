use crate::prelude::*;

pub async fn init_db() -> (PgPool, PgPool) {
    let db_url = env::var("DB_URL").expect("DB url not set");
    let neon_eu_url = env::var("NEON_EU").expect("Neon EU url not set");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Failed to connect to the database");

    let neon_eu = PgPoolOptions::new()
        .max_connections(100)
        .connect(&neon_eu_url)
        .await
        .expect("Failed to connect to the database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    (pool, neon_eu)
}
