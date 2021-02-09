mod database;
use database::{User, connect_sql};
use actix_files::{NamedFile};
use actix_web::{web, App, HttpServer, Result, HttpResponse, Responder};
use std::error;

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(
	User
	{
	    username: "test".to_string(),
	    following: vec!["teste".to_string(), "testeee".to_string()],
	}
    ))   
}



#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let pool = connect_sql();

    
    
    println!("Listening on port 8080");
    HttpServer::new(|| {
	App::new().route("/", web::get().to(index))
    })
	.bind("127.0.0.1:8080")?
	.run()
	.await
}
