use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

pub type DbPool = PgPool;

pub async fn init_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await?;

    // sqlx::migrate!("./migrations")
    //     .run(&pool)
    //     .await
    //     .map_err(|e| {
    //         eprintln!("Migration failed: {}", e);
    //         e
    //     })?;

    Ok(pool)
}
