use crate::prelude::*;

pub async fn init_db() {
    let db_url = env::var("DB_URL").expect("DB url not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to the database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");
}
