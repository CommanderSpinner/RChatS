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
use sqlx::Error;

#[derive(Template)]
#[template(path = "index.html")]
struct HtmlTemplate<'a> {
    title: &'a str,
    site_content: &'a str,
    msg: &'a str,
}

pub async fn page(State(conn): State<Arc<Connection>>, Form(payload): Form<HashMap<String, String>>) -> Html<String> {
    let page: HtmlTemplate;

    let username: String = payload.get("user_name").cloned().unwrap_or_default();
    let password: String = payload.get("plain_password").cloned().unwrap_or_default();
    let action = payload.get("action").map(|s| s.as_str()).unwrap_or_default();

    common::debug_println!("username: {}", username);
    common::debug_println!("password: {}", password);

    if conn.validate_login(&username, &password).await.unwrap_or(false) {
        common::debug_println!("loging in");
        page = HtmlTemplate {
            title: "web acess",
            site_content: "interface",
            msg: "",
        }
    } else if action == "create_account" { 
        common::debug_println!("creating account"); 
        let mut title = "web access";
        let mut  site_content = "interface";
        let mut msg = "";

        if let Err(e) = conn.create_user(
            username,
            password,
        ).await {
            eprintln!("failed to create user: {e}");

            if let Error::Database(db_err) = &e { // this error code should be refactored to be in connection class
                eprintln!("code: {:?}", db_err.code()); // Option<&str>
                eprintln!("message: {}", db_err.message());

                if db_err.code().as_deref() == Some("23505") {
                    msg = "User already taken";
                }

            }

            title = "login";
            site_content = "login";
        }

        page = HtmlTemplate {
            title: title,
            site_content: site_content,
            msg: msg,
        }

        // maybe send code 303 back (prg)

    } else {
        page = HtmlTemplate {
            title: "login",
            site_content: "login",
            msg: "",
        }
    }

    Html(page.render().unwrap())
}
