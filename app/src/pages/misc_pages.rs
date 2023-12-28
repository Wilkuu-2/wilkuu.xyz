use axum::routing::Router;
use axum::routing::get;
use axum::response::IntoResponse;
use axum::extract::Path; 
use views::home::*;
use askama_axum::into_response;

pub fn misc_pages() -> Router {
    Router::new()
        .route("/", get(homepage))
        .route("/greet/:name", get(greet))
}



async fn homepage() -> impl IntoResponse
{
    into_response(&HomePage {})
}

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    let template = HelloTemplate {name};
    into_response(&template)
}


