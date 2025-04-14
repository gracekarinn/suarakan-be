use diesel::prelude::*;
use diesel::PgConnection;
use chrono::Local;
use crate::model::update::*;
use crate::schema::updates;

pub struct UpdateService;

impl UpdateService {
    pub fn get_update_by_id(
        conn: &mut PgConnection,
        update_id: i32
    ) -> Result<Update, diesel::result::Error> {
        updates::table.find(update_id).first(conn)
    }

    pub fn update_status(
        conn: &mut PgConnection,
        update_id: i32,
        admin_id: i64,
        status: String,
        remarks: Option<String>,
        proof: Option<String>
    ) -> Result<Update, diesel::result::Error> {
        // Check if status is valid
        let valid_statuses = vec!["Received", "Processing", "Completed", "Rejected"];
        if !valid_statuses.contains(&status.as_str()) {
            return Err(diesel::result::Error::RollbackTransaction);
        }
        
        let previous_update = updates::table.find(update_id).first::<Update>(conn)?;
        let previous_created_at = previous_update.createdat.clone();

        let update = Update {
            updateid: update_id,
            createdat: previous_created_at,
            updatedat: Some(Local::now().naive_local()),
            remarks,
            proof,
            status: Some(status),
            adminid: Some(admin_id),
        };
        
        diesel::update(updates::table.find(update_id))
            .set(&update)
            .get_result(conn)
    }
}