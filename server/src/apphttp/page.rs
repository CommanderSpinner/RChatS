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
    
    common::debug_println!("web interface access");

    let username: String = payload.get("user_name").cloned().unwrap_or_default();
    let password: String = payload.get("plain_password").cloned().unwrap_or_default();

    let create_account: bool = payload.get("create_account")
        .map(|s| s.as_str())              // Convert Option<String> to Option<&str>
        .unwrap_or("false")               // Default to string "false"
        .parse()                          // Now you can parse the &str
        .unwrap_or(false);

    common::debug_println!("username: {}", username);
    common::debug_println!("password: {}", password);
    common::debug_println!("creating account: {}", create_account);

    if !username.is_empty() && !password.is_empty() { // change later to validation. if true display web interface
        

        page = HtmlTemplate {
            title: "web acess",
            site_content: "interface",
        }
    } else if create_account {
            conn.create_user(username, hash_password(&password).expect("something went wrong hashing"));
            page = HtmlTemplate {
                title: "account creation",
                site_content: "create",
            }

    } else { 
        page = HtmlTemplate {
            title: "login rchats",
            site_content: "login",
        };
    }

    Html(page.render().unwrap())
}
