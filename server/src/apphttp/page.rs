use askama::Template;
use axum::{
    extract::State,
    extract::Form,
    response::Html,
    routing::{get, post},
    Router,
};

use axum_extra::extract::cookie::{Cookie, CookieJar};

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
    username: String,
}

pub async fn page(State(conn): State<Arc<Connection>>, jar: CookieJar, Form(payload): Form<HashMap<String, String>>) -> (CookieJar, Html<String>) {
    let mut page: HtmlTemplate;
    let mut jar = jar;

    let username = jar
        .get("username")
        .map(|c| c.value().to_string())
        .unwrap_or_else(|| {
            payload
                .get("user_name")
                .cloned()
                .unwrap_or_default()
        });

    let password = jar
        .get("password")
        .map(|c| c.value().to_string())
        .unwrap_or_else(|| {
            payload
                .get("plain_password")
                .cloned()
                .unwrap_or_default()
        });

    let action = payload.get("action").map(|s| s.as_str()).unwrap_or_default();

    common::debug_println!("username: {}", username);
    common::debug_println!("password: {}", password);
    common::debug_println!("action: {}", action);

    // auto login if credentials are set
    if action != "login" && !username.is_empty() && !password.is_empty() && action != "sign_out" {
        if conn.validate_login(&username, &password).await.unwrap_or(false) {
            page = HtmlTemplate {
                title: "web access",
                site_content: "interface",
                msg: "",
                username: username.clone().to_string(),
            };

            return (jar, Html(page.render().unwrap()));
        }
    }

    if action == "login" {
        if conn.validate_login(&username, &password).await.unwrap_or(false){
            page = HtmlTemplate {
                title: "web acess",
                site_content: "interface",
                msg: "",
                username: username.clone().to_string(),
            };

            //is for username propably will be replaced by session id later
            let uid = match conn.get_uid(&username).await {
                Ok(uid) => uid,
                Err(err) => {
                    eprintln!("get_uid failed: {}", err);
                    page.msg = "DB error";

                    return (jar, Html(page.render().unwrap()));
                }
            };
            common::debug_println!("SETTING COOKIE UID: {}", uid);

            // using cookies for now and later session id
            jar = jar
                .add(
                    Cookie::build(("username", username.clone()))
                        .path("/")
                        .http_only(false)
                        .build()
                )
                .add(
                    Cookie::build(("password", password.clone()))
                        .path("/")
                        .http_only(false)
                        .build()
                )
                .add(
                    Cookie::build((("userid", uid.to_string())))
                        .path("/")
                        .http_only(false)
                        .build()
                );
            
        } else {
            page = HtmlTemplate {
                title: "login",
                site_content: "login",
                msg: "Wrong username or password",
                username: "".to_string(),
            };
        }
    } else if action == "create_account" { 
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
                } else {
                    msg = "unknown error";
                }

            }

            title = "login";
            site_content = "login";
        }

        page = HtmlTemplate {
            title: title,
            site_content: site_content,
            msg: msg,
                username: "".to_string(),
        }

        // maybe send code 303 back (prg)

    } else if action == "sign_out" {
        jar = jar
            .remove(Cookie::named("username"))
            .remove(Cookie::named("password"))
            .remove(Cookie::named("userid"));

        page = HtmlTemplate {
            title: "login",
            site_content: "login",
            msg: "",
            username: "".to_string(),
        }
    } else {
        page = HtmlTemplate {
            title: "login",
            site_content: "login",
            msg: "",
            username: "".to_string(),
        }
    }

    (jar, Html(page.render().unwrap()))
}
