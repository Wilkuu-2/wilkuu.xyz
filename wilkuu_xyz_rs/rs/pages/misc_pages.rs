
use crate::HtmlTemplate; 
use askama::Template; 
use axum::routing::Router;
use axum::routing::get;
use axum::response::IntoResponse;
use axum::extract::Path;

pub fn misc_pages() -> Router {
    Router::new()
        .route("/", get(homepage))
        .route("/greet/:name", get(greet))
}


// Homepage
#[derive(Template)]
#[template(path="home.html")]
struct HomePage {}

async fn homepage() -> impl IntoResponse
{
    HtmlTemplate(HomePage {})
}

#[derive(Template)]
#[template(ext="html",source="<h1>Hello, {{ name }}!</h1><h2>How are you?</h2>")]
struct HelloTemplate {name: String} 

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    let template = HelloTemplate {name};
    HtmlTemplate(template)
}


