use serde::{Serialize, Deserialize};
use serde_json::json;
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
    let query =
	sqlx::query!(
	    r#"
INSERT INTO posts ( id, title, content )
VALUES ( $1, $2, $3 )
RETURNING id"#, post.uuid_id, post.title, post.content
	).fetch_one(pool).await?;
    
    Ok(query.id)
}

pub async fn delete_post (pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    let query =
	sqlx::query!(
	    r#"
DELETE FROM posts WHERE id = $1"#, id).fetch_one(pool).await?;
    Ok(())
}

pub async fn update_post (pool: &PgPool, post: Post) -> Result<(), sqlx::Error> {
    let query =
	sqlx::query!(
	    r#"
UPDATE posts
SET title = $1, content = $2
WHERE id = $3"#, post.title, post.content, post.uuid_id).fetch_one(pool).await?;
    Ok(())
}

pub async fn read_post(pool: &PgPool, title: String) -> Result<String, sqlx::Error> {
    let query =
	sqlx::query!(
	    r#"
SELECT * from posts where title = $1
	"#,title).fetch_one(pool).await?;

    Ok(query.content)
}
