pub mod database;
use serde::Serialize;
use uuid::Uuid;
use typed_html::{dom::*, text, html, elements::*};
pub use database::{Post, connect_sql, delete_post, create_post};



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

pub fn render_head() -> Box<dyn MetadataContent<String>> {
	html! {
	    <meta charset="UTF-8" />: String
	}
}

pub fn render_navbar() -> Box<div<String>> {
	html!{
	    <div id="navbar" >
		<a href="/">"Home"</a>
		</div>
	}
}
