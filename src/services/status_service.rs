use diesel::{prelude::*, result::Error};
use crate::{
    model::report::{Report, NewReport, UpdateReport},
    database::connection::PgPool,
    schema::reports::dsl::*,
};

pub struct ReportService;

impl ReportService {
    pub fn create_report(new_report: NewReport, pool: &PgPool) -> Result<Report, Error> {
        let mut conn = pool.get().unwrap();
        
        diesel::insert_into(reports)
            .values(&new_report)
            .get_result::<Report>(&mut conn)
    }

    pub fn get_report(report_id: i32, pool: &PgPool) -> Result<Report, Error> {
        let mut conn = pool.get().unwrap();
        reports.find(report_id).first::<Report>(&mut conn)
    }

    pub fn update_report(
        report_id: i32,
        update_data: UpdateReport,
        pool: &PgPool
    ) -> Result<Report, Error> {
        let mut conn = pool.get().unwrap();
        
        diesel::update(reports.find(report_id))
            .set(&update_data)
            .get_result::<Report>(&mut conn)
    }

    pub fn delete_report(report_id: i32, pool: &PgPool) -> Result<usize, Error> {
        let mut conn = pool.get().unwrap();
        
        diesel::delete(reports.find(report_id))
            .execute(&mut conn)
    }
}