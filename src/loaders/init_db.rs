use crate::prelude::*;

pub async fn init_db() -> (PgPool, PgPool, PgPool, PgPool) {
    let db_url = env::var("DB_URL").expect("DB url not set");
    let neon_eu_url = env::var("NEON_EU").expect("Neon EU url not set");
    let neon_us_east_url = env::var("NEON_US_EAST").expect("Neon US East url not set");
    let neon_us_west_url = env::var("NEON_US_WEST").expect("Neon US West url not set");

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
    let neon_us_east = PgPoolOptions::new()
        .max_connections(100)
        .connect(&neon_us_east_url)
        .await
        .expect("Failed to connect to the database");
    let neon_us_west = PgPoolOptions::new()
        .max_connections(100)
        .connect(&neon_us_west_url)
        .await
        .expect("Failed to connect to the database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    (pool, neon_eu, neon_us_east, neon_us_west)
}
