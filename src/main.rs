use std::error::Error;

use sea_orm::{Database, DatabaseConnection};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();

    info!("Connecting to db...");
    let db: DatabaseConnection = Database::connect("sqlite::memory:").await?;

    Ok(())
}
