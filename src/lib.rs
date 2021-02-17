mod database;
use serde::Serialize;
use uuid::Uuid;
use database::{connect_sql, Post, create_post};


pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
    let pool = connect_sql().await?;
    let somePost =
	Post {
	    uuid_id: Uuid::new_v4(),
	    title: String::from("test"),
	    content: String::from("This is an example of speech"),
	};
    create_post(&pool, somePost).await?;
    Ok(())
}
