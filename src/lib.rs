mod database;
use serde::Serialize;
use uuid::Uuid;
use database::{connect_sql, Post, delete_post};


pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
    let pool = connect_sql().await?;
    delete_post(&pool, Uuid::parse_str("47e29637-b9a6-4b88-bdf1-95a1b9a83e1c")?);
   create_post(&pool, somePost).await?;
    Ok(())
}
