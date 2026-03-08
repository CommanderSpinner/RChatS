use askama::Template;
use axum::response::Html;
use tower_cookies::{Cookie, Cookies};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    title: &'a str,
}

pub async fn index() -> Html<String> {
    let page = IndexTemplate {
        title: "Login",
    };

    common::debug_println!("web interface accessed");

    Html(page.render().unwrap())
}