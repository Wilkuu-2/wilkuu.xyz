use askama::Template;
use dotenv::dotenv;
use sea_orm::Database;
use std::env;
use std::net::SocketAddr;
use tracing::{event, Level};

mod controllers;
mod db;
mod util;

#[tokio::main]
pub async fn start() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    event!(Level::INFO, "Starting");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    let state = AppState::new(AppStateT { conn });

    let app = Router::new()
        .nest("/portfolio", controllers::posts())
        .merge(controllers::misc_pages())
        .with_state(state);

    let addr_string: String = env::args().nth(1).expect("No address given");

    let addr: SocketAddr = addr_string
        .parse()
        .expect(&format!("Failed to parse ip: {} \n", addr_string));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub type Router = axum::routing::Router<AppState, axum::body::Body>;
pub type AppState = std::sync::Arc<AppStateT>;
pub struct AppStateT {
    conn: sea_orm::DatabaseConnection,
}

pub fn view<T: Template>(
    t: &T,
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    match t.render() {
        Ok(body) => {
            let headers = [(
                axum::http::header::CONTENT_TYPE,
                axum::http::HeaderValue::from_static(T::MIME_TYPE),
            )];
            Ok((headers, body))
        }
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
