use diesel::prelude::*;
use diesel::PgConnection;
use crate::model::report::*;
use crate::schema::reports;

pub struct ReportService;

impl ReportService {
    pub fn create_report(
        conn: &mut PgConnection, 
        new_report: NewReport
    ) -> Result<Report, diesel::result::Error> {
        diesel::insert_into(reports::table)
            .values(&new_report)
            .get_result(conn)
    }

    pub fn get_all_reports(conn: &mut PgConnection) -> Result<Vec<Report>, diesel::result::Error> {
        reports::table.load(conn)
    }

    pub fn get_reports_by_reporter(conn: &mut PgConnection, reporter_id: i64) -> Result<Vec<Report>, diesel::result::Error> {
        reports::table
            .filter(reports::reporterid.eq(reporter_id))
            .load(conn)
    }

    pub fn get_report_by_id(conn: &mut PgConnection, report_id: i32) -> Result<Report, diesel::result::Error> {
        reports::table.find(report_id).first(conn)
    }

    pub fn update_report(
        conn: &mut PgConnection, 
        report_id: i32,
        report: Report
    ) -> Result<Report, diesel::result::Error> {
        diesel::update(reports::table.find(report_id))
            .set(&report)
            .get_result(conn)
    }

    pub fn delete_report(conn: &mut PgConnection, report_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(reports::table.find(report_id))
            .execute(conn)
    }
}