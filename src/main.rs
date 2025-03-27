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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let pool = establish_connection_pool()?;
    println!("Connected to PostgreSQL");

    let app = routes::create_routes(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}