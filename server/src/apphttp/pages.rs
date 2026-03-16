use askama::Template;
use axum::{
    extract::Form,
    response::Html,
    routing::{get, post},
    Router,
};
//use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::Deserialize;
use std::net::SocketAddr;
use std::collections::HashMap;

use crate::connection::Connection;

#[derive(Template)]
#[template(path = "index.html")]
struct htmlTemplate<'a> {
    title: &'a str,
    site_content: &'a str,
}

pub async fn login() -> Html<String> {
    let page = htmlTemplate {
        title: "login rchats",
        site_content: "form",
    };

    common::debug_println!("web login access");

    Html(page.render().unwrap())
}

pub async fn interface(Form(payload): Form<HashMap<String, String>>) -> Html<String> {

    let page = htmlTemplate {
        title: "web acess",
        site_content: "interface",
    };

    let username: String = payload.get("user_name").cloned().unwrap_or_default();
    let password: String = payload.get("plain_password").cloned().unwrap_or_default();

    common::debug_println!("username: {}", username);
    common::debug_println!("password: {}", password);

    Html(page.render().unwrap())
}
