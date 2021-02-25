mod database;
use uuid::Uuid;
use actix_files::{NamedFile};
use cms_actix::create_test_post;
use actix_web::{web, App, HttpServer, Result, HttpResponse, Responder, Error};
use actix_web::http::StatusCode;
use std::error;
use database::{Post, create_post, connect_sql};

// async fn index() -> Result<HttpResponse> {
//     Ok(HttpResponse::Ok().json(
// 	User
// 	{
// 	    username: "test".to_string(),
// 	    following: vec!["teste".to_string(), "testeee".to_string()],
// 	}
//     ))   
// }


pub async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK).body("Hello World, Rust!"))
}


#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Listening on port 8080");
    Ok(())
}
