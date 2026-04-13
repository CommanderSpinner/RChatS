use askama::Template;
use axum::{
    extract::State,
    extract::Form,
    response::Html,
    routing::{get, post},
    Router,
};

//use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::Deserialize;
use std::net::SocketAddr;
use std::collections::HashMap;
use std::sync::Arc;
use crate::connection::Connection;

#[derive(Template)]
#[template(path = "index.html")]
struct HtmlTemplate<'a> {
    title: &'a str,
    site_content: &'a str,
}

/*
pub async fn login() -> Html<String> {
    let page = htmlTemplate {
    };

    common::debug_println!("web login access");

    Html(page.render().unwrap())
}
*/

pub async fn page(State(conn): State<Arc<Connection>>, Form(payload): Form<HashMap<String, String>>) -> Html<String> {
    let page: HtmlTemplate;
    
    common::debug_println!("web interface access");

    let username: String = payload.get("user_name").cloned().unwrap_or_default();
    let password: String = payload.get("plain_password").cloned().unwrap_or_default();

    common::debug_println!("username: {}", username);
    common::debug_println!("password: {}", password);

    // if cookies are empty login page is displayed
    if username.is_empty() && password.is_empty() { // change later to validation of credentials
        page = HtmlTemplate {
            title: "login rchats",
            site_content: "form",
        }
    } else { // otherwise the web interface
        page = HtmlTemplate {
            title: "web acess",
            site_content: "interface",
        }
    }

    Html(page.render().unwrap())
}
