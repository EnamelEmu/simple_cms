pub mod database;
use serde::Serialize;
use uuid::Uuid;
use typed_html::{dom::*, text, html, elements::*};
pub use database::{Post, connect_sql, delete_post, create_post};
use std::fs::{read, File};
use std::path::Path;
use std::env::temp_dir;
use std::io::Read;
use std::process::Command;

pub async fn create_test_post(payload: Post) {

    let poole = connect_sql().await.unwrap();
    let _ = create_post(&poole, payload).await;

}

pub fn render_post (post: Post) -> DOMTree<std::string::String> {
    html!(
	<html>
	    <head>
	    <title>{text!("{}", post.title)}</title>
	{ render_head() }
	</head>
	    <body>
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
	html!(<link rel="stylesheet" href="https://unpkg.com/chota@latest" />),
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
	    <img src="/assets/drawing.svg" />
	    </a>
	    </div>
	    </nav>
    }
}

// pub fn save_post(path: Path, post_html: DOMTree<String>) -> Result<()> {
//     fs::write(path, post_html.to_string())?;
// }

pub fn input_from_erditor() -> Result<String, std::io::Error> {
    let editor = var("EDITOR").unwrap();
    let mut file_path = temp_dir();
    file_path.push("temp_post");
    File::create(&file_path).expect("err");

    Command::new(editor).arg(&file_path).status().expect();

    let mut content = String::new();
    File::open(file_path).expect("err").read_to_string(&mut content);

    return content;
}
