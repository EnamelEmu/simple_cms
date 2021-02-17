use cms_actix::test;
use actix_files::{NamedFile};
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
    let _ = test().await;
    
    
    println!("Listening on port 8080");
    HttpServer::new(|| {
	App::new().route("/", web::get().to(index))
    })
	.bind("127.0.0.1:8080")?
	.run()
	.await
}
