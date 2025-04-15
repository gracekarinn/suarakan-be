use diesel::prelude::*;
use diesel::PgConnection;
use crate::model::update::*;
use crate::schema::updates;

pub struct UpdateService;

impl UpdateService {
    pub fn create_update(
        conn: &mut PgConnection, 
        new_update: NewUpdate
    ) -> Result<Update, diesel::result::Error> {
        diesel::insert_into(updates::table)
            .values(&new_update)
            .get_result(conn)
    }

    pub fn get_update_by_id(conn: &mut PgConnection, update_id: i32) -> Result<Update, diesel::result::Error> {
        updates::table.find(update_id).first(conn)
    }

    pub fn get_update_by_report_id(conn: &mut PgConnection, report_id: i32) -> Result<Update, diesel::result::Error> {
        updates::table
            .filter(updates::reportid.eq(report_id))
            .first(conn)
    }

    pub fn update_update(
        conn: &mut PgConnection, 
        update_id: i32,
        update: Update
    ) -> Result<Update, diesel::result::Error> {
        diesel::update(updates::table.find(update_id))
            .set(&update)
            .get_result(conn)
    }

    pub fn delete_update_by_id(conn: &mut PgConnection, update_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(updates::table.find(update_id))
            .execute(conn)
    }

    pub fn delete_update_by_report_id(conn: &mut PgConnection, report_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(updates::table.filter(updates::reportid.eq(report_id)))
            .execute(conn)
    }
}