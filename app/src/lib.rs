use std::net::SocketAddr;
use axum::Router;
use std::env; 
use dotenv::dotenv;
use tracing::{event,Level};

mod pages;
mod db;
mod util;

#[tokio::main]
pub async fn start() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    event!(Level::INFO, "Starting");

    let app = Router::new()
        .merge(pages::misc_pages());
    
    let addr_string: String = env::args()
        .nth(1)
        .expect("No address given");
    
    let addr: SocketAddr = addr_string 
        .parse()
        .expect(&format!("Failed to parse ip: {} \n", addr_string));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}




