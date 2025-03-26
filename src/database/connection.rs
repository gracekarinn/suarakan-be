use tokio_postgres::{NoTls, Error};
use std::env;

pub async fn connect_to_db() -> Result<tokio_postgres::Client, Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });
    
    Ok(client)
}