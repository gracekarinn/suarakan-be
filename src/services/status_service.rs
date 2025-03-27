use diesel::prelude::*;
use diesel::PgConnection;
use crate::model::update::{Update, NewUpdate};
use crate::schema::updates;
use chrono::Utc;

pub struct StatusService;

impl StatusService {
    /// Membuat status laporan baru
    pub fn create_status(
        conn: &mut PgConnection,
        data_id: i32,
        remarks: Option<String>,
        proof: Option<String>,
        status: String,
        admin_id: i32,
    ) -> Result<Update, diesel::result::Error> {
        use crate::schema::updates::dsl::*;

        let new_status = NewUpdate {
            dataid: data_id,
            createdat: Some(Utc::now().naive_utc()),
            updatedat: None,
            remarks,
            proof,
            status: Some(status),
            adminid: Some(admin_id),
        };

        diesel::insert_into(updates)
            .values(&new_status)
            .get_result(conn)
    }

    /// Membaca status laporan berdasarkan ID
    pub fn get_status_by_id(
        conn: &mut PgConnection,
        update_id: i32,
    ) -> Result<Update, diesel::result::Error> {
        use crate::schema::updates::dsl::*;

        updates.find(update_id).first(conn)
    }
}
