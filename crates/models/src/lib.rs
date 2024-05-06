pub mod collective;
pub mod mate;

use sqlx::{migrate::MigrateError, PgPool};

pub async fn migrate(pool: &PgPool) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}
