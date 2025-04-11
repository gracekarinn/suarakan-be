use diesel::prelude::*;
use chrono::Local;
use crate::model::{
    report::{NewReport, Report},
    reporter::{NewReporter, Reporter},
    incident::{NewIncident, Incident},
    victim::{NewVictim, Victim},
    accused::{NewAccused, Accused},
    proof::{NewProof, Proof},
    specialized_reports::{NewUIReport, NewPerempuanReport, NewHamReport}
};
use crate::schema::{
    reports, reporters, incidents, victims, accused, proofs, 
    ui_reports, perempuan_reports, ham_reports
};
use anyhow::{Result, Context};

#[derive(Debug, Deserialize)]
pub struct ReportFormData {
    // Pelapor
    reporter_phone: String,
    reporter_job: String,
    reporter_date_of_birth: String,
    reporter_address: String,
    reporter_relationship: String,

    // Pelanggaran
    violation_location: String,
    violation_time: String,
    violation_description: String,
    victim_needs: Vec<String>,
    past_effort: String,
    evidence_link: Option<String>,

    // Korban
    victim_full_name: String,
    victim_nik: Option<String>,
    victim_email: Option<String>,
    victim_domicile_address: Option<String>,
    victim_phone: Option<String>,
    victim_job: Option<String>,
    victim_gender: String,
    victim_date_of_birth: Option<String>,
    victim_place_of_birth: Option<String>,
    victim_education: Option<String>,
    victim_marital_status: Option<String>,
    victim_special_needs: Option<bool>,
    victim_disability_description: Option<String>,

    // Tersangka
    suspect_full_name: String,
    suspect_email: Option<String>,
    suspect_domicile_address: Option<String>,
    suspect_phone: Option<String>,
    suspect_job: Option<String>,
    suspect_gender: String,
    suspect_date_of_birth: Option<String>,
    suspect_place_of_birth: Option<String>,
    suspect_education: Option<String>,
    suspect_relationship: String,

    // Level Pengaduan
    reporting_level: String
}

pub struct ReportService;

impl ReportService {
    pub async fn create_report(
        conn: &mut PgConnection, 
        form_data: ReportFormData
    ) -> Result<Report> {
        conn.transaction(|conn| {
            let new_proof = NewProof {
                link: form_data.evidence_link.unwrap_or_default()
            };
            let proof = diesel::insert_into(proofs::table)
                .values(&new_proof)
                .get_result::<Proof>(conn)
                .context("Failed to insert proof")?;

            let new_reporter = NewReporter {
                reporterid: 0, 
                phonenum: Some(form_data.reporter_phone),
                occupation: Some(form_data.reporter_job),
                dateofbirth: form_data.reporter_date_of_birth.parse().ok(),
                officialaddress: Some(form_data.reporter_address),
                relationship: Some(form_data.reporter_relationship),
                faxnum: None
            };
            let reporter = diesel::insert_into(reporters::table)
                .values(&new_reporter)
                .get_result::<Reporter>(conn)
                .context("Failed to insert reporter")?;

            let new_incident = NewIncident {
                location: form_data.violation_location,
                time: Local::now().naive_local(),
                description: Some(form_data.violation_description),
                victimneeds: Some(form_data.victim_needs.join(", ")),
                pasteffort: Some(form_data.past_effort)
            };
            let incident = diesel::insert_into(incidents::table)
                .values(&new_incident)
                .get_result::<Incident>(conn)
                .context("Failed to insert incident")?;

            let new_victim = NewVictim {
                fullname: form_data.victim_full_name,
                nik: form_data.victim_nik,
                email: form_data.victim_email,
                domicileaddress: form_data.victim_domicile_address,
                phonenum: form_data.victim_phone,
                occupation: form_data.victim_job,
                sex: Some(form_data.victim_gender),
                dateofbirth: form_data.victim_date_of_birth.and_then(|d| d.parse().ok()),
                placeofbirth: form_data.victim_place_of_birth,
                officialaddress: None,
                educationlevel: form_data.victim_education,
                faxnum: None,
                marriagestatus: form_data.victim_marital_status,
                marriageage: None,
                isuploaded: Some(false),
                disability: form_data.victim_disability_description
            };
            let victim = diesel::insert_into(victims::table)
                .values(&new_victim)
                .get_result::<Victim>(conn)
                .context("Failed to insert victim")?;

            let new_accused = NewAccused {
                fullname: form_data.suspect_full_name,
                email: form_data.suspect_email,
                domicileaddress: form_data.suspect_domicile_address,
                phonenum: form_data.suspect_phone,
                occupation: form_data.suspect_job,
                sex: Some(form_data.suspect_gender),
                dateofbirth: form_data.suspect_date_of_birth.and_then(|d| d.parse().ok()),
                placeofbirth: form_data.suspect_place_of_birth,
                educationlevel: form_data.suspect_education,
                relationship: Some(form_data.suspect_relationship)
            };
            let accused = diesel::insert_into(accused::table)
                .values(&new_accused)
                .get_result::<Accused>(conn)
                .context("Failed to insert accused")?;

            let new_report = NewReport {
                reporterid: reporter.reporterid,
                createdat: Some(Local::now().naive_local()),
                updatedat: None,
                proofid: Some(proof.proofid),
                incidentid: Some(incident.incidentid),
                victimid: Some(victim.victimid),
                accusedid: Some(accused.accusedid)
            };
            let report = diesel::insert_into(reports::table)
                .values(&new_report)
                .get_result::<Report>(conn)
                .context("Failed to insert report")?;

            match form_data.reporting_level.as_str() {
                "Universitas Indonesia" => {
                    diesel::insert_into(ui_reports::table)
                        .values(&NewUIReport { 
                            reportid: report.reportid, 
                            updateid: None 
                        })
                        .execute(conn)?;
                },
                "Komnas HAM" => {
                    diesel::insert_into(ham_reports::table)
                        .values(&NewHamReport { 
                            reportid: report.reportid, 
                            updateid: None 
                        })
                        .execute(conn)?;
                },
                "Komnas Perempuan" => {
                    diesel::insert_into(perempuan_reports::table)
                        .values(&NewPerempuanReport { 
                            reportid: report.reportid, 
                            updateid: None 
                        })
                        .execute(conn)?;
                },
                _ => return Err(anyhow::anyhow!("Invalid reporting level"))
            }

            Ok(report)
        })
    }
}