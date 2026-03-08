use askama::Template;
use axum::response::Html;
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    title: &'a str,
}

pub async fn index(jar: CookieJar) -> Html<String> {
    let page = IndexTemplate {
        title: "login rchats",
    };

    common::debug_println!("web interface accessed");

    Html(page.render().unwrap())
}