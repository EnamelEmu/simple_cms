#![recursion_limit = "512"]
use uuid::Uuid;
use actix_files::{NamedFile};
use cms_actix::*;
use actix_web::{web, App, HttpServer, Result, HttpResponse, Responder, Error};
use actix_web::http::StatusCode;
use std::error;
use structopt::StructOpt;

// async fn index() -> Result<HttpResponse> {
//     Ok(HttpResponse::Ok().json(
// 	User
// 	{
// 	    username: "test".to_string(),
// 	    following: vec!["teste".to_string(), "testeee".to_string()],
// 	}
//     ))   
// }

#[derive(StructOpt, Debug)]
pub enum Cli {
    /// Creates a new post
    New {
	/// Title for the post
	#[structopt(short = "t", long = "title")]
	new_title: std::string::String,
    },
    ///Delete a post
    Delete {
	/// Title for the post to be deleted
	#[structopt(short = "t", long = "title")]
	delete_title: std::string::String
    }
}

//////////////////////////////////////////////////////////////////
// #[tokio::main]					        //
// async fn main() -> Result<(), Box<dyn std::error::Error>> {  //
//     let args = Cli::from_args();			        //
//     let paths: Vec<String> = read_lines(args.address_file)?; //
//     fetch(paths, args.time_out.into()).await?;	        //
//     Ok(())						        //
// }							        //
//////////////////////////////////////////////////////////////////


pub async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK).body("Hello World, Rust!"))
}


#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    match Cli::from_args() {
	Cli::New { new_title } => {
	    input_from_editor(new_title).await;
	}
	Cli::Delete { delete_title } => {}
    }

    println!("Listening on port 8080");
    
    Ok(())
}
