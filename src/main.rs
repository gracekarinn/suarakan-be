mod database;
mod auth;
mod model;
mod schema;
mod middleware;
mod routes;

use dotenv::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let client = database::connect_to_db().await?;
    println!("Connected to PostgreSQL");

    let app = routes::create_routes();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}