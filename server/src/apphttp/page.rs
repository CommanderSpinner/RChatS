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
use crate::connection::hash_password;

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

    let username: String = payload.get("user_name").cloned().unwrap_or_default();
    let password: String = payload.get("plain_password").cloned().unwrap_or_default();
    let action = payload.get("action").map(|s| s.as_str()).unwrap_or_default();

    common::debug_println!("username: {}", username);
    common::debug_println!("password: {}", password);

    if action == "login" { // change later to validation. if credentials are valid display interfae page
        common::debug_println!("loging in");
        page = HtmlTemplate {
            title: "web acess",
            site_content: "interface",
        }
    } else if action == "create_account" { 
        common::debug_println!("creating account"); 
        if let Err(e) = conn.create_user(
            username,
            hash_password(&password).expect("something went wrong hashing"),
        ).await {
            eprintln!("failed to create user: {e}");
        }
        page = HtmlTemplate {
            title: "account creation",
            site_content: "interface",
        }

        // maybe send code 303 back (prg)

    } else {
        page = HtmlTemplate {
            title: "web acess",
            site_content: "login",
        }
    }

    Html(page.render().unwrap())
}
