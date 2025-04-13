use diesel::prelude::*;
use diesel::PgConnection;
use crate::model::update::{Update, NewUpdate};
use chrono::Utc;

pub struct StatusService;

impl StatusService {
    pub fn create_status(
        conn: &mut PgConnection,
        data_id: i32,
        remarks_param: Option<String>,
        proof_param: Option<String>,
        status_param: String,
        admin_id: i64,
    ) -> Result<Update, diesel::result::Error> {
        use crate::schema::updates::dsl::*;

        if data_id <= 0 {
            eprintln!("Invalid data_id: {}", data_id);
            return Err(diesel::result::Error::RollbackTransaction);
        }

        let new_status = NewUpdate {
            dataid: data_id,
            createdat: Some(Utc::now().naive_utc()),
            updatedat: None,
            remarks: remarks_param.map(|r| r.trim().to_string()),
            proof: proof_param.map(|p| p.trim().to_string()),
            status: Some(status_param.trim().to_string()),
            adminid: Some(admin_id),
        };

        diesel::insert_into(updates)
            .values(&new_status)
            .get_result(conn)
    }

    pub fn get_status_by_id(
        conn: &mut PgConnection,
        update_id: i32,
    ) -> Result<Update, diesel::result::Error> {
        use crate::schema::updates::dsl::*;

        updates.find(update_id).first(conn)
    }

    pub fn update_status(
        conn: &mut PgConnection,
        update_id: i32,
        remarks_param: Option<String>,
        proof_param: Option<String>,
        status_param: Option<String>,
        admin_id: i64,
    ) -> Result<Update, diesel::result::Error> {
        use crate::schema::updates::dsl::*;

        diesel::update(updates.find(update_id))
            .set((
                updatedat.eq(Some(Utc::now().naive_utc())),
                remarks.eq(remarks_param.map(|r| r.trim().to_string())),
                proof.eq(proof_param.map(|p| p.trim().to_string())),
                status.eq(status_param.map(|s| s.trim().to_string())),
                adminid.eq(Some(admin_id)),
            ))
            .get_result(conn)
    }

    pub fn list_statuses(
        conn: &mut PgConnection,
        page: i64,
        per_page: i64,
    ) -> Result<Vec<Update>, diesel::result::Error> {
        use crate::schema::updates::dsl::*;

        let offset = (page - 1) * per_page;

        updates
            .order(createdat.desc())
            .limit(per_page)
            .offset(offset)
            .load::<Update>(conn)
    }
}