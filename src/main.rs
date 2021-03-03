use uuid::Uuid;
use actix_files::{NamedFile};
use cms_actix::{render_post, create_test_post, Post};
use actix_web::{web, App, HttpServer, Result, HttpResponse, Responder, Error};
use actix_web::http::StatusCode;
use std::error;

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
    println!("{:#?}",
	     render_post(Post {
		 uuid_id: Uuid::new_v4(),
		 title: String::from("Title :D"),
		 content: String::from("CONTENT"),
	     }
	     ).to_string()
    );
    Ok(())
}
