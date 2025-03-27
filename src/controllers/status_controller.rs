use actix_web::{web, HttpResponse};
use chrono::Utc;
use crate::{
    database::connection::PgPool,
    model::report::{Report, NewReport, UpdateReport},
    services::report_service::ReportService
};

#[derive(serde::Deserialize)]
pub struct CreateReportRequest {
    pub reporterid: i32,
    pub proofid: Option<i32>,
    pub incidentid: Option<i32>,
    pub victimid: Option<i32>,
    pub accusedid: Option<i32>,
}

#[derive(serde::Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::reports)]
pub struct UpdateReportRequest {
    pub proofid: Option<i32>,
    pub incidentid: Option<i32>,
    pub victimid: Option<i32>,
    pub accusedid: Option<i32>,
}

// CREATE
pub async fn create_report(
    report_data: web::Json<CreateReportRequest>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let new_report = NewReport {
        reporterid: report_data.reporterid,
        createdat: Some(Utc::now().naive_utc()),
        updatedat: None,
        proofid: report_data.proofid,
        incidentid: report_data.incidentid,
        victimid: report_data.victimid,
        accusedid: report_data.accusedid,
    };

    match ReportService::create_report(new_report, &pool) {
        Ok(created_report) => HttpResponse::Created().json(created_report),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

// READ
pub async fn get_report(
    path: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let report_id = path.into_inner();
    
    match ReportService::get_report(report_id, &pool) {
        Ok(report) => HttpResponse::Ok().json(report),
        Err(diesel::result::Error::NotFound) => 
            HttpResponse::NotFound().body("Report not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// UPDATE
pub async fn update_report(
    path: web::Path<i32>,
    update_data: web::Json<UpdateReportRequest>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let report_id = path.into_inner();
    let update_report = UpdateReport {
        updatedat: Some(Utc::now().naive_utc()),
        proofid: update_data.proofid,
        incidentid: update_data.incidentid,
        victimid: update_data.victimid,
        accusedid: update_data.accusedid,
    };

    match ReportService::update_report(report_id, update_report, &pool) {
        Ok(updated_report) => HttpResponse::Ok().json(updated_report),
        Err(diesel::result::Error::NotFound) => 
            HttpResponse::NotFound().body("Report not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// DELETE
pub async fn delete_report(
    path: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let report_id = path.into_inner();
    
    match ReportService::delete_report(report_id, &pool) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(diesel::result::Error::NotFound) => 
            HttpResponse::NotFound().body("Report not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}