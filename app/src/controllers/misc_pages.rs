use axum::routing::get;
use axum::response::IntoResponse;
use axum::extract::Path; 
use views::home::*;
use crate::{view,Router}; 

pub fn misc_pages() -> Router {
    Router::new()
        .route("/", get(homepage))
        .route("/greet/:name", get(greet))
}

async fn homepage() -> impl IntoResponse
{
    view(&HomePage {})
}

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    let template = HelloTemplate {name};
    view(&template)
}


