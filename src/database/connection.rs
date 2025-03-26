use postgres::{Client, NoTls};
use std::env;

pub fn connect_to_db() -> Result<Client, postgres::Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    Client::connect(&database_url, NoTls)
}