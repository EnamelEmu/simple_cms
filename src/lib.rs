mod database;
use serde::Serialize;
use uuid::Uuid;
use database::{connect_sql, Post, delete_post, create_post};


pub async fn create_test_post(payload: Post) {

    let poole = connect_sql().await.unwrap();
    let _ = create_post(&poole, payload).await;

}
