use axum::{
    extract::{Path, State},
    http::{Request, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::Local;
use serde_json::json;
use regex::Regex;

use crate::{
    database::connection::DbPool,
    middleware::auth::extract_token_from_request,
    model::report::{NewReport, Report},
    model::update::NewUpdate,
    services::report_service::ReportService,
    services::update_service::UpdateService,
};

fn sanitize_string(input: Option<String>) -> Option<String> {
    input.map(|s| html_escape::encode_text(&s).to_string())
}

fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}

fn is_valid_phone(phone: &str) -> bool {
    let phone_regex = Regex::new(r"^\d{8,13}$").unwrap();
    phone_regex.is_match(phone)
}

fn is_valid_nik(nik: &str) -> bool {
    let nik_regex = Regex::new(r"^\d{16}$").unwrap();
    nik_regex.is_match(nik)
}

fn is_valid_sex(sex: &str) -> bool {
    matches!(sex, "Laki-laki" | "Perempuan" | "Lainnya")
}

fn is_valid_relationship(relationship: &str) -> bool {
    const VALID_RELATIONSHIPS: [&str; 16] = [
        "Pasangan", "Mantan Pasangan", "Kekasih", "Ayah Kandung", "Ibu Kandung",
        "Ayah Tiri", "Ibu Tiri", "Saudara Kandung", "Saudara Tiri", "Keluarga",
        "Keluarga Tiri", "Keluarga Ipar", "Tetangga", "Teman", "Rekan Kerja", "Orang Tak Dikenal"
    ];
    VALID_RELATIONSHIPS.contains(&relationship)
}

fn is_valid_url(url: &str) -> bool {
    let url_regex = Regex::new(r"^(https?://)?([a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}(/[^\s]*)?$").unwrap();
    url_regex.is_match(url)
}

fn is_valid_marriage_status(status: &str) -> bool {
    const VALID_STATUSES: [&str; 5] = [
        "Belum Kawin", "Kawin Belum Tercatat", "Kawin Tercatat", "Cerai Hidup", "Cerai Mati"
    ];
    VALID_STATUSES.contains(&status)
}

fn is_valid_authority(authority: &str) -> bool {
    const VALID_AUTHORITIES: [&str; 3] = [
        "Universitas Indonesia", "Komnas HAM", "Komnas Perempuan"
    ];
    VALID_AUTHORITIES.contains(&authority)
}

fn is_valid_education_level(level: &str) -> bool {
    const VALID_LEVELS: [&str; 8] = [
        "Tidak Sekolah", "SD / MI Sederajat", "SMP / MTs Sederajat", 
        "SMA / MA / SMK Sederajat", "Diploma (D1/D2/D3)", "Sarjana (S1/D4)", 
        "Magister (S2)", "Doktor (S3)"
    ];
    VALID_LEVELS.contains(&level)
}

fn validate_report(report: &NewReport) -> Result<(), (StatusCode, &'static str)> {
    if report.incidentlocation.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Lokasi insiden harus diisi!"));
    }
    
    if report.victimfullname.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Nama korban harus diisi!"));
    }
    
    if report.accusedfullname.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Nama pelaku harus diisi!"));
    }
    
    if report.authority.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Tujuan pengaduan harus diisi!"));
    }
    
    if let Some(email) = &report.victimemail {
        if !email.is_empty() && !is_valid_email(email) {
            return Err((StatusCode::BAD_REQUEST, "Format email tidak valid"));
        }
    }
    
    if let Some(phone) = &report.reporterphonenum {
        if !phone.is_empty() && !is_valid_phone(phone) {
            return Err((StatusCode::BAD_REQUEST, "Nomor telepon harus 8 - 13 digit"));
        }
    }
    
    if let Some(phone) = &report.victimphonenum {
        if !phone.is_empty() && !is_valid_phone(phone) {
            return Err((StatusCode::BAD_REQUEST, "Nomor telepon harus 8 - 13 digit"));
        }
    }
    
    if let Some(phone) = &report.accusedphonenum {
        if !phone.is_empty() && !is_valid_phone(phone) {
            return Err((StatusCode::BAD_REQUEST, "Nomor telepon harus 8 - 13 digit"));
        }
    }
    
    if let Some(nik) = &report.victimnik {
        if !nik.is_empty() && !is_valid_nik(nik) {
            return Err((StatusCode::BAD_REQUEST, "NIK harus 16 digit"));
        }
    }
    
    if let Some(sex) = &report.victimsex {
        if !sex.is_empty() && !is_valid_sex(sex) {
            return Err((StatusCode::BAD_REQUEST, "Jenis kelamin harus 'Laki-laki', 'Perempuan', atau 'Lainnya'"));
        }
    }
    
    if let Some(sex) = &report.accusedsex {
        if !sex.is_empty() && !is_valid_sex(sex) {
            return Err((StatusCode::BAD_REQUEST, "Jenis kelamin harus 'Laki-laki', 'Perempuan', atau 'Lainnya'"));
        }
    }
    
    if let Some(rel) = &report.reporterrelationship {
        if !rel.is_empty() && !is_valid_relationship(rel) {
            return Err((StatusCode::BAD_REQUEST, "Hubungan tidak valid"));
        }
    }
    
    if let Some(rel) = &report.accusedrelationship {
        if !rel.is_empty() && !is_valid_relationship(rel) {
            return Err((StatusCode::BAD_REQUEST, "Hubungan tidak valid"));
        }
    }
    
    if let Some(proof) = &report.incidentproof {
        if proof.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "Bukti insiden harus diisi!"));
        }
        if !is_valid_url(proof) {
            return Err((StatusCode::BAD_REQUEST, "Bukti insiden harus berupa format link yang valid!"));
        }
    } else {
        return Err((StatusCode::BAD_REQUEST, "Bukti insiden harus diisi!"));
    }
    
    if let Some(status) = &report.victimmarriagestatus {
        if !status.is_empty() && !is_valid_marriage_status(status) {
            return Err((StatusCode::BAD_REQUEST, "Status pernikahan tidak valid"));
        }
    }
    
    if !is_valid_authority(&report.authority) {
        return Err((StatusCode::BAD_REQUEST, "Tujuan pengaduan tidak valid"));
    }
    
    if let Some(level) = &report.victimeducationlevel {
        if !level.is_empty() && !is_valid_education_level(level) {
            return Err((StatusCode::BAD_REQUEST, "Edukasi tidak valid"));
        }
    }
    
    Ok(())
}

// SANITASI XSS
fn sanitize_report(mut report: NewReport) -> NewReport {
    report.reporterfullname = sanitize_string(report.reporterfullname);
    report.reporterphonenum = sanitize_string(report.reporterphonenum);
    report.reporteraddress = sanitize_string(report.reporteraddress);
    report.reporterrelationship = sanitize_string(report.reporterrelationship);
    report.incidentlocation = sanitize_string(Some(report.incidentlocation)).unwrap_or_default();
    report.incidentdescription = sanitize_string(report.incidentdescription);
    report.incidentvictimneeds = sanitize_string(report.incidentvictimneeds);
    report.incidentproof = sanitize_string(report.incidentproof);
    report.victimfullname = sanitize_string(Some(report.victimfullname)).unwrap_or_default();
    report.victimnik = sanitize_string(report.victimnik);
    report.victimemail = sanitize_string(report.victimemail);
    report.victimaddress = sanitize_string(report.victimaddress);
    report.victimphonenum = sanitize_string(report.victimphonenum);
    report.victimoccupation = sanitize_string(report.victimoccupation);
    report.victimsex = sanitize_string(report.victimsex);
    report.victimplaceofbirth = sanitize_string(report.victimplaceofbirth);
    report.victimeducationlevel = sanitize_string(report.victimeducationlevel);
    report.victimmarriagestatus = sanitize_string(report.victimmarriagestatus);
    report.accusedfullname = sanitize_string(Some(report.accusedfullname)).unwrap_or_default();
    report.accusedaddress = sanitize_string(report.accusedaddress);
    report.accusedphonenum = sanitize_string(report.accusedphonenum);
    report.accusedoccupation = sanitize_string(report.accusedoccupation);
    report.accusedsex = sanitize_string(report.accusedsex);
    report.accusedrelationship = sanitize_string(report.accusedrelationship);
    report.authority = sanitize_string(Some(report.authority)).unwrap_or_default();
    
    report
}

pub async fn create_report(
    State(pool): State<DbPool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<NewReport>,
) -> impl IntoResponse {
    let mut builder = Request::builder();
    for (key, value) in headers.iter() {
        builder = builder.header(key, value);
    }

    match extract_token_from_request(&builder.body(()).unwrap()) {
        Ok(claims) => {
            if claims.user_type != "PELAPOR" {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "Only PELAPOR can create reports"})),
                ).into_response();
            }

            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Database connection error"})),
                    ).into_response();
                }
            };

            match validate_report(&payload) {
                Ok(_) => {},
                Err((status, message)) => {
                    return (
                        status,
                        Json(json!({"error": message})),
                    ).into_response();
                }
            }

            let mut new_report = sanitize_report(payload);
            new_report.createdat = Some(Local::now().naive_local());
            new_report.reporterid = Some(claims.user_id);

            match ReportService::create_report(&mut conn, new_report) {
                Ok(report) => {
                    let new_update = NewUpdate {
                        createdat: Local::now().naive_local(),
                        updatedat: None,
                        remarks: Some(String::from("")),
                        proof: Some(String::from("")),
                        status: Some(String::from("Received")),
                        reportid: report.reportid,
                    };

                    match UpdateService::create_update(&mut conn, new_update) {
                        Ok(_) => {
                            (StatusCode::CREATED, Json(json!(report))).into_response()
                        }
                        Err(e) => {
                            let _ = ReportService::delete_report(&mut conn, report.reportid);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({"error": format!("Failed to create update: {}", e.to_string())})),
                            ).into_response()
                        }
                    }
                }
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                ).into_response(),
            }
        }
        Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
    }
}

pub async fn get_reports(
    State(pool): State<DbPool>,
    req: Request<axum::body::Body>,
) -> impl IntoResponse {
    match extract_token_from_request(&req) {
        Ok(claims) => {
            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Database connection error"})),
                    ).into_response()
                }
            };

            let reports = match claims.user_type.as_str() {
                "ADMIN" => {
                    match ReportService::get_all_reports(&mut conn) {
                        Ok(reports) => reports,
                        Err(e) => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({"error": e.to_string()})),
                            ).into_response()
                        }
                    }
                }
                "PELAPOR" => {
                    match ReportService::get_reports_by_reporter(&mut conn, claims.user_id) {
                        Ok(reports) => reports,
                        Err(e) => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({"error": e.to_string()})),
                            ).into_response()
                        }
                    }
                }
                _ => {
                    return (
                        StatusCode::FORBIDDEN,
                        Json(json!({"error": "Unauthorized access"})),
                    ).into_response()
                }
            };

            let mut reports_with_updates = Vec::new();
            for report in reports {
                match UpdateService::get_update_by_report_id(&mut conn, report.reportid) {
                    Ok(update) => {
                        reports_with_updates.push(json!({
                            "report": report,
                            "update": update
                        }));
                    }
                    Err(e) => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({"error": format!("Failed to get update: {}", e.to_string())})),
                        ).into_response()
                    }
                }
            }

            (StatusCode::OK, Json(json!(reports_with_updates))).into_response()
        }
        Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
    }
}

pub async fn get_report(
    State(pool): State<DbPool>,
    Path(report_id): Path<i32>,
    req: Request<axum::body::Body>,
) -> impl IntoResponse {
    match extract_token_from_request(&req) {
        Ok(claims) => {
            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Database connection error"})),
                    ).into_response()
                }
            };

            match ReportService::get_report_by_id(&mut conn, report_id) {
                Ok(report) => {
                    if claims.user_type == "PELAPOR" && report.reporterid != Some(claims.user_id) {
                        return (
                            StatusCode::FORBIDDEN,
                            Json(json!({"error": "You are not authorized to view this report"})),
                        ).into_response();
                    }

                    match UpdateService::get_update_by_report_id(&mut conn, report_id) {
                        Ok(update) => {
                            (StatusCode::OK, Json(json!({
                                "report": report,
                                "update": update
                            }))).into_response()
                        }
                        Err(e) => {
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({"error": format!("Failed to get update: {}", e.to_string())})),
                            ).into_response()
                        }
                    }
                }
                Err(diesel::result::Error::NotFound) => (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Report not found"})),
                ).into_response(),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                ).into_response(),
            }
        }
        Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
    }
}

pub async fn update_report(
    State(pool): State<DbPool>,
    Path(report_id): Path<i32>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<Report>,
) -> impl IntoResponse {
    let mut builder = Request::builder();
    for (key, value) in headers.iter() {
        builder = builder.header(key, value);
    }

    match extract_token_from_request(&builder.body(()).unwrap()) {
        Ok(claims) => {
            if claims.user_type != "PELAPOR" {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "Only PELAPOR can update reports"})),
                ).into_response();
            }

            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Database connection error"})),
                    ).into_response();
                }
            };

            let existing = match ReportService::get_report_by_id(&mut conn, report_id) {
                Ok(report) => report,
                Err(diesel::result::Error::NotFound) => {
                    return (
                        StatusCode::NOT_FOUND,
                        Json(json!({"error": "Report not found"})),
                    ).into_response();
                }
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": e.to_string()})),
                    ).into_response();
                }
            };

            if existing.reporterid != Some(claims.user_id) {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "You are not authorized to update this report"})),
                ).into_response();
            }

            match UpdateService::get_update_by_report_id(&mut conn, report_id) {
                Ok(update) => {
                    if update.status != Some(String::from("Received")) {
                        return (
                            StatusCode::FORBIDDEN,
                            Json(json!({"error": "Report can only be updated when status is 'Received'"})),
                        ).into_response();
                    }
                }
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": format!("Failed to get update: {}", e.to_string())})),
                    ).into_response();
                }
            }

            // VALIDASI NEWREPORT
            let new_report = NewReport {
                createdat: payload.createdat,
                updatedat: payload.updatedat,
                reporterfullname: payload.reporterfullname.clone(),
                reporterphonenum: payload.reporterphonenum.clone(),
                reporteraddress: payload.reporteraddress.clone(),
                reporterrelationship: payload.reporterrelationship.clone(),
                incidentlocation: payload.incidentlocation.clone(),
                incidenttime: payload.incidenttime,
                incidentdescription: payload.incidentdescription.clone(),
                incidentvictimneeds: payload.incidentvictimneeds.clone(),
                incidentproof: payload.incidentproof.clone(),
                victimfullname: payload.victimfullname.clone(),
                victimnik: payload.victimnik.clone(),
                victimemail: payload.victimemail.clone(),
                victimaddress: payload.victimaddress.clone(),
                victimphonenum: payload.victimphonenum.clone(),
                victimoccupation: payload.victimoccupation.clone(),
                victimsex: payload.victimsex.clone(),
                victimdateofbirth: payload.victimdateofbirth,
                victimplaceofbirth: payload.victimplaceofbirth.clone(),
                victimeducationlevel: payload.victimeducationlevel.clone(),
                victimmarriagestatus: payload.victimmarriagestatus.clone(),
                accusedfullname: payload.accusedfullname.clone(),
                accusedaddress: payload.accusedaddress.clone(),
                accusedphonenum: payload.accusedphonenum.clone(),
                accusedoccupation: payload.accusedoccupation.clone(),
                accusedsex: payload.accusedsex.clone(),
                accusedrelationship: payload.accusedrelationship.clone(),
                authority: payload.authority.clone(),
                reporterid: payload.reporterid,
            };

            // VALIDASI REPORT DATA
            match validate_report(&new_report) {
                Ok(_) => {},
                Err((status, message)) => {
                    return (
                        status,
                        Json(json!({"error": message})),
                    ).into_response();
                }
            }

            // SANITASI
            let mut updated_report = payload;
            updated_report.reportid = existing.reportid;
            updated_report.createdat = existing.createdat;
            updated_report.updatedat = Some(Local::now().naive_local());
            updated_report.reporterid = Some(claims.user_id);
            
            updated_report.reporterfullname = sanitize_string(updated_report.reporterfullname);
            updated_report.reporterphonenum = sanitize_string(updated_report.reporterphonenum);
            updated_report.reporteraddress = sanitize_string(updated_report.reporteraddress);
            updated_report.reporterrelationship = sanitize_string(updated_report.reporterrelationship);
            updated_report.incidentlocation = sanitize_string(Some(updated_report.incidentlocation)).unwrap_or_default();
            updated_report.incidentdescription = sanitize_string(updated_report.incidentdescription);
            updated_report.incidentvictimneeds = sanitize_string(updated_report.incidentvictimneeds);
            updated_report.incidentproof = sanitize_string(updated_report.incidentproof);
            updated_report.victimfullname = sanitize_string(Some(updated_report.victimfullname)).unwrap_or_default();
            updated_report.victimnik = sanitize_string(updated_report.victimnik);
            updated_report.victimemail = sanitize_string(updated_report.victimemail);
            updated_report.victimaddress = sanitize_string(updated_report.victimaddress);
            updated_report.victimphonenum = sanitize_string(updated_report.victimphonenum);
            updated_report.victimoccupation = sanitize_string(updated_report.victimoccupation);
            updated_report.victimsex = sanitize_string(updated_report.victimsex);
            updated_report.victimplaceofbirth = sanitize_string(updated_report.victimplaceofbirth);
            updated_report.victimeducationlevel = sanitize_string(updated_report.victimeducationlevel);
            updated_report.victimmarriagestatus = sanitize_string(updated_report.victimmarriagestatus);
            updated_report.accusedfullname = sanitize_string(Some(updated_report.accusedfullname)).unwrap_or_default();
            updated_report.accusedaddress = sanitize_string(updated_report.accusedaddress);
            updated_report.accusedphonenum = sanitize_string(updated_report.accusedphonenum);
            updated_report.accusedoccupation = sanitize_string(updated_report.accusedoccupation);
            updated_report.accusedsex = sanitize_string(updated_report.accusedsex);
            updated_report.accusedrelationship = sanitize_string(updated_report.accusedrelationship);
            updated_report.authority = sanitize_string(Some(updated_report.authority)).unwrap_or_default();

            match ReportService::update_report(&mut conn, report_id, updated_report) {
                Ok(report) => {
                    (StatusCode::OK, Json(json!(report))).into_response()
                }
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                ).into_response(),
            }
        }
        Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
    }
}

pub async fn delete_report(
    State(pool): State<DbPool>,
    Path(report_id): Path<i32>,
    req: Request<axum::body::Body>,
) -> impl IntoResponse {
    match extract_token_from_request(&req) {
        Ok(claims) => {
            if claims.user_type != "PELAPOR" {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "Only PELAPOR can delete reports"})),
                ).into_response();
            }

            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Database connection error"})),
                    ).into_response()
                }
            };

            let existing = match ReportService::get_report_by_id(&mut conn, report_id) {
                Ok(report) => report,
                Err(diesel::result::Error::NotFound) => {
                    return (
                        StatusCode::NOT_FOUND,
                        Json(json!({"error": "Report not found"})),
                    ).into_response();
                }
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": e.to_string()})),
                    ).into_response();
                }
            };

            if existing.reporterid != Some(claims.user_id) {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "You are not authorized to delete this report"})),
                ).into_response();
            }

            match UpdateService::get_update_by_report_id(&mut conn, report_id) {
                Ok(update) => {
                    if update.status != Some(String::from("Received")) && update.status != Some(String::from("Rejected")) {
                        return (
                            StatusCode::FORBIDDEN,
                            Json(json!({"error": "Report can only be deleted when status is 'Received' or 'Rejected'"})),
                        ).into_response();
                    }
                }
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": format!("Failed to get update: {}", e.to_string())})),
                    ).into_response();
                }
            }

            match UpdateService::delete_update_by_report_id(&mut conn, report_id) {
                Ok(_) => {
                    match ReportService::delete_report(&mut conn, report_id) {
                        Ok(_) => (
                            StatusCode::OK,
                            Json(json!({"message": "Report and associated update deleted successfully"})),
                        ).into_response(),
                        Err(e) => (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({"error": format!("Failed to delete report: {}", e.to_string())})),
                        ).into_response(),
                    }
                }
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("Failed to delete update: {}", e.to_string())})),
                ).into_response(),
            }
        }
        Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
    }
}