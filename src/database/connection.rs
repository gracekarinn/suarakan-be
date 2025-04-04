use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::Connection;
use diesel::ConnectionError;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub async fn connect_to_db() -> Result<PgConnection, ConnectionError> {
    let database_url = get_database_url();
    PgConnection::establish(&database_url)
}

pub fn establish_connection_pool() -> Result<DbPool, Box<dyn std::error::Error>> {
    let database_url = get_database_url();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)?;
    Ok(pool)
}

fn get_database_url() -> String {
    let is_production = env::var("RUST_ENV").map(|env| env == "production").unwrap_or(false);
    
    if is_production {
        let db_name = env::var("DB_NAME").expect("DB_NAME must be set in production");
        let db_user = env::var("DB_USER").expect("DB_USER must be set in production");
        let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set in production");
        let db_host = env::var("DB_HOST").expect("DB_HOST must be set in production");
        let db_port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
        
        format!("postgres://{}:{}@{}:{}/{}", db_user, db_password, db_host, db_port, db_name)
    } else {
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file for local development")
    }
}