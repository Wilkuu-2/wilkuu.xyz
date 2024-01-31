use crate::{view, Router};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::get;
use views::home::*;

pub fn misc_pages() -> Router {
    Router::new()
        .route("index", "/", get(homepage))
        .route("greet","/greet/:name", get(greet))
}

async fn homepage() -> impl IntoResponse {
    view(&HomePage {})
}

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    let template = HelloTemplate { name };
    view(&template)
}
