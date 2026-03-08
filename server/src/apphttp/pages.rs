use askama::Template;
use axum::{
    extract::Form,
    response::Html,
    routing::{get, post},
    Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::Deserialize;
use std::net::SocketAddr;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    title: &'a str,
}

pub async fn index(jar: CookieJar, Form(payload): Form<HashMap<String, String>>) -> Html<String> {

    let username: String = payload.get("username").cloned().unwrap_or_default();
    let password: String = payload.get("password").cloned().unwrap_or_default();

    common::debug_println!("username: {}", username);
    common::debug_println!("password: {}", password);
    

    let page = IndexTemplate {
        title: "login rchats",
    };

    common::debug_println!("web interface accessed");

    Html(page.render().unwrap())
}