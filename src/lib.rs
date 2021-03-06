#![recursion_limit = "512"]
pub mod database;
use serde::Serialize;
use uuid::Uuid;
use typed_html::{dom::*, unsafe_text, text, html, elements::*};
pub use database::{Post, connect_sql, delete_post, create_post};
use std::fs::{read, File};
use std::path::Path;
use std::env::{var, temp_dir};
use std::io::Read;
use std::process::Command;
use std::path::PathBuf;
use std::fs;
use futures::executor; // 0.3.1

pub async fn create_test_post(payload: Post) {

    let poole = connect_sql().await.unwrap();
    let _ = create_post(&poole, payload).await;

}

pub fn render_index() -> DOMTree<String> {
    let mut dire = fs::read_dir("./cms/").unwrap();
    html!(
	<html>
	    <head>
	    <title>{text!("Index")}</title>
	{ render_head() }
	</head>
	    <body>
	<ul>
	{
	    dire.map(|res| res.map(|e|
				   html!(<li>
					 {
					     let e = e.path();
					     unsafe_text!("<a href={:?}>{:?}</a>", e, e)}
		     </li>)).unwrap())
	}
	</ul>
	    </body>
	    </html>
    )
}

    
pub fn render_post (post: &Post) -> DOMTree<std::string::String> {
    html!(
	<html>
	    <head>
	    <title>{text!("{}", post.title)}</title>
	{ render_head() }
	</head>
	    <body class="max-width mx-auto px3 ltr">
	{ render_navbar() }
	<div id="post">
	    <h1>{text!("{}", post.title)}</h1>
	    <h3>{text!("{}", post.content)}</h3>
	    </div>
	    </body>
	    </html>
    )
        
}

pub fn render_head() -> Vec<Box<dyn MetadataContent<String>>> {
    vec![
	html!(<meta charset="UTF-8" />),
	html!(<link rel="stylesheet" href="https://nee.lv/css/style.css" />),
    ]
}

pub fn render_navbar() -> Box<nav<String>> {
    html!{
	<nav class="nav">
	    <div id="nav-left" >
	    <a href="/">"Home"</a>
	    </div>

	    <div class="nav-center">
	    <a href="/">
//	    <img width="30" src="../assets/drawing.svg" />
	    </a>
	    </div>
	    </nav>
    }
}

// pub fn save_post(path: Path, post_html: DOMTree<String>) -> Result<()> {
//     fs::write(path, post_html.to_string())?;
// }

pub async fn input_from_editor(title: std::string::String) -> Result<(), std::io::Error> {
    let editor = var("EDITOR").unwrap();
    let mut file_path = PathBuf::from("./cms/");
    file_path.push(format!("{}.html",title));
    File::create(&file_path).expect("err");

    Command::new(editor).arg(&file_path).status().expect("err");

    

    let mut content = String::new();
    File::open(&file_path).expect("err").read_to_string(&mut content);
    let post = Post::new(title, content);
    let final_html = render_post(&post);
    fs::write(&file_path, final_html.to_string());

    let index_html = render_index();
    fs::write("./index.html", index_html.to_string());

    let poole = connect_sql().await.unwrap();
    let _ = create_post(&poole, post).await;
    
    Ok(())

}
