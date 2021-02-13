use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct Post {
    pub uuid_id: Uuid,
    pub title: String,
    pub content: String,
}
