use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::Connection;
use diesel::ConnectionError;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub async fn connect_to_db() -> Result<PgConnection, ConnectionError> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    PgConnection::establish(&database_url)
}

pub fn establish_connection_pool() -> Result<DbPool, Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)?;
    
    Ok(pool)
}