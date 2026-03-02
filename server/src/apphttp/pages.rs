use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    title: &'a str,
}

pub async fn index() -> Html<String> {
    let page = IndexTemplate {
        title: "Login",
    };

    Html(page.render().unwrap())
}