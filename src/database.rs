use serde::{Serialize, Deserialize};
use time::{Date, Time};
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres;
use sqlx::PgPool;
use sqlx::Error;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Post {
    pub uuid_id: Uuid,
    pub title: String,
    pub content: String,
}

pub async fn connect_sql() -> Result<sqlx::PgPool, sqlx::Error> {
    return Ok(PgPoolOptions::new()
	.max_connections(5)
	.connect("postgres://postgres:test@localhost:5432/cms")
	.await?);
}

pub async fn create_post (pool: &PgPool, post: Post) -> Result<Uuid, sqlx::Error> {
    let rec =
	sqlx::query!(
	    r#"
INSERT INTO posts ( title, content )
VALUES ( $1, $2 )
RETURNING id"#, post.title, post.content
	).fetch_one(pool)
	.await?;
    
    Ok(rec.id)
}

pub async fn delete_post () {
    todo!()
}

pub async fn update_post () {
    todo!()
}
