pub mod utils;

use suarakan_be::database::DbPool;
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        dotenv::dotenv().ok();
    });
}

pub fn get_mock_db_pool() -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new("postgres://fake:fake@localhost/fake");
    r2d2::Pool::builder()
        .max_size(1)
        .test_on_check_out(false) 
        .build_unchecked(manager) 
}

pub async fn setup_test_db() -> DbPool {
    get_mock_db_pool()
}