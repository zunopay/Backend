pub mod entity;

use crate::{config::config, error::Result};
use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;

/*
 * 1. DatabaseConnection is Clone + Send + Sync, so sharing across tasks is safe.
 * 2. Database::connect will manage multiple pool connections internally
 */
pub async fn connect_database() -> Result<DatabaseConnection> {
    let url = &config().DB_URL;
    let db = Database::connect(url).await?;

    Ok(db)
}
