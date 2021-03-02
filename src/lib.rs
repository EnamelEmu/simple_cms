mod database;
use serde::Serialize;
use uuid::Uuid;
use typed_html::{dom::*, text, html};
use database::{connect_sql, Post, delete_post, create_post};


pub async fn create_test_post(payload: Post) {

    let poole = connect_sql().await.unwrap();
    let _ = create_post(&poole, payload).await;

}

pub fn render_post (post: &Post) -> std::string::String {
    let mut dom: DOMTree<String> =
	html!(
	    <div id="post">
		<h1>{text!("{}", post.title)}</h1>
		<h3>{text!("{}", post.content)}</h3>
		
	</div>
	);
    let doc_str = dom.to_string();
    Ok(doc_str)
        
}

pub async fn render_home() {
    todo!();
}
