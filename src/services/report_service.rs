// src/services/report_service.rs
use diesel::prelude::*;
use diesel::PgConnection;
use chrono::Local;
use crate::model::report::*;
use crate::model::update::*;
use crate::schema::{reports, updates};

pub struct ReportService;

impl ReportService {
    pub fn create_report(
        conn: &mut PgConnection, 
        mut new_report: NewReport,
        reporter_id: i32
    ) -> Result<Report, diesel::result::Error> {
        // Start a transaction
        conn.transaction(|conn| {
            // Create a new update with default values
            let new_update = NewUpdate {
                updateid: 0, // Will be auto-generated
                createdat: Local::now().naive_local(),
                updatedat: None,
                remarks: Some("".to_string()),
                proof: Some("".to_string()),
                status: Some("Received".to_string()),
                adminid: None,
            };

            // Insert the new update and get the generated ID
            let update: Update = diesel::insert_into(updates::table)
                .values(&new_update)
                .get_result(conn)?;

            // Set the update ID and reporter ID in the new report
            new_report.updateid = update.updateid;
            new_report.reporterid = reporter_id;
            new_report.createdat = Some(Local::now().naive_local());

            // Insert the new report
            diesel::insert_into(reports::table)
                .values(&new_report)
                .get_result(conn)
        })
    }

    pub fn get_reports_for_admin(conn: &mut PgConnection) -> Result<Vec<Report>, diesel::result::Error> {
        reports::table
            .order(reports::createdat.desc())
            .load::<Report>(conn)
    }

    pub fn get_reports_for_reporter(
        conn: &mut PgConnection,
        reporter_id: i32
    ) -> Result<Vec<Report>, diesel::result::Error> {
        reports::table
            .filter(reports::reporterid.eq(reporter_id))
            .order(reports::createdat.desc())
            .load::<Report>(conn)
    }

    pub fn get_report_by_id(
        conn: &mut PgConnection,
        report_id: i32
    ) -> Result<Report, diesel::result::Error> {
        reports::table.find(report_id).first(conn)
    }

    pub fn update_report(
        conn: &mut PgConnection,
        report_id: i32,
        reporter_id: i32,
        updated_report: Report
    ) -> Result<Report, diesel::result::Error> {
        // Check if report belongs to reporter and is in "Received" status
        let report = reports::table
            .find(report_id)
            .first::<Report>(conn)?;
        
        if report.reporterid != reporter_id {
            return Err(diesel::result::Error::NotFound);
        }
        
        // Get the update associated with this report
        let update = updates::table
            .find(report.updateid)
            .first::<Update>(conn)?;
            
        if update.status.as_deref() != Some("Received") {
            return Err(diesel::result::Error::RollbackTransaction);
        }
        
        // Update the report
        diesel::update(reports::table.find(report_id))
            .set(&updated_report)
            .get_result(conn)
    }

    pub fn delete_report(
        conn: &mut PgConnection,
        report_id: i32,
        reporter_id: i32
    ) -> Result<usize, diesel::result::Error> {
        // Check if report belongs to reporter and is in "Received" or "Rejected" status
        let report = reports::table
            .find(report_id)
            .first::<Report>(conn)?;
        
        if report.reporterid != reporter_id {
            return Err(diesel::result::Error::NotFound);
        }
        
        // Get the update associated with this report
        let update = updates::table
            .find(report.updateid)
            .first::<Update>(conn)?;
            
        let status = update.status.as_deref();
        if status != Some("Received") && status != Some("Rejected") {
            return Err(diesel::result::Error::RollbackTransaction);
        }
        
        // Start a transaction
        conn.transaction(|conn| {
            // Delete the update
            diesel::delete(updates::table.find(report.updateid))
                .execute(conn)?;
                
            // Delete the report
            diesel::delete(reports::table.find(report_id))
                .execute(conn)
        })
    }
}