mod database;
mod auth;
mod model;
mod schema;
mod middleware;
mod routes;
mod controllers;
mod services;

use dotenv::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use database::connection::establish_connection_pool;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = establish_connection_pool()?;
    println!("Connected to PostgreSQL");

    let app = routes::create_routes(pool);

    let port = std::env::var("PORT").unwrap_or_else(|_| "80".to_string());
    let port = port.parse::<u16>().unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port)); 
    println!("Server running on http://{}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}