mod database;

use dotenv::dotenv;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut client = database::connect_to_db()?;

    let rows = client.query("SELECT 42", &[])?;
    let value: i32 = rows[0].get(0);
    
    println!("Connected to PostgreSQL");
    
    Ok(())
}