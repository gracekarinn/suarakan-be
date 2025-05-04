use jsonwebtoken::{encode, Header, EncodingKey};
use suarakan_be::auth::jwt::JwtClaims;
use chrono::{Utc, Duration};
use std::env;

pub fn generate_test_token(user_id: i64, user_type: &str, full_name: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .expect("valid timestamp")
        .timestamp();

    
    let claims = JwtClaims {
        user_id,
        user_type: user_type.to_string(),
        full_name: full_name.to_string(),
        token_type: "access".to_string(),
        exp: expiration as usize,
        email: "test@example.com".to_string(),
        iat: Utc::now().timestamp() as usize,
        is_email_verified: false,
        jti: "unique_token_id".to_string()
    };

    let secret = env::var("PODS_JWT_SECRET").unwrap_or_else(|_| "test_secret".to_string());
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("Failed to generate token")
}

pub fn get_admin_token() -> String {
    generate_test_token(1, "ADMIN", "Test Admin")
}

pub fn get_reporter_token() -> String {
    generate_test_token(2, "PELAPOR", "Test Reporter")
}

pub fn get_invalid_token() -> String {
    "invalid.jwt.token".to_string()
}

pub mod test_data {
    use suarakan_be::model::publication::NewPublication;
    use suarakan_be::model::report::{NewReport, Report};
    use suarakan_be::model::update::NewUpdate;
    use chrono::{Local, NaiveDate, NaiveDateTime};

    pub fn sample_publication() -> NewPublication {
        NewPublication {
            title: "Test Publication".to_string(),
            createdat: Local::now().naive_local(),
            updatedat: None,
            description: Some("Test Description".to_string()),
            filelink: Some("www.google.com".to_string()),
            adminid: Some(1),
        }
    }

    pub fn sample_report() -> NewReport {
        NewReport {
            createdat: Some(Local::now().naive_local()),
            updatedat: None,
            reporterfullname: Some("Test Reporter".to_string()),
            reporterphonenum: Some("081234567890".to_string()),
            reporteraddress: Some("Test Address".to_string()),
            reporterrelationship: Some("Keluarga".to_string()),
            incidentlocation: "Test Incident Location".to_string(),
            incidenttime: Local::now().naive_local(),
            incidentdescription: Some("Test incident description".to_string()),
            incidentvictimneeds: Some("Test victim needs".to_string()),
            incidentproof: Some("www.google.com".to_string()),
            victimfullname: "Test Victim".to_string(),
            victimnik: Some("1234567890123456".to_string()),
            victimemail: Some("victim@example.com".to_string()),
            victimaddress: Some("Victim Address".to_string()),
            victimphonenum: Some("087654321098".to_string()),
            victimoccupation: Some("Student".to_string()),
            victimsex: Some("Perempuan".to_string()),
            victimdateofbirth: Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
            victimplaceofbirth: Some("Test City".to_string()),
            victimeducationlevel: Some("Sarjana (S1/D4)".to_string()),
            victimmarriagestatus: Some("Belum Kawin".to_string()),
            accusedfullname: "Test Accused".to_string(),
            accusedaddress: Some("Accused Address".to_string()),
            accusedphonenum: Some("089876543210".to_string()),
            accusedoccupation: Some("Worker".to_string()),
            accusedsex: Some("Laki-laki".to_string()),
            accusedrelationship: Some("Teman".to_string()),
            authority: "Universitas Indonesia".to_string(),
            reporterid: Some(2),
        }
    }

    pub fn sample_update() -> NewUpdate {
        NewUpdate {
            createdat: Local::now().naive_local(),
            updatedat: None,
            remarks: Some("Initial review".to_string()),
            proof: Some("www.google.com".to_string()),
            status: Some("Received".to_string()),
            reportid: 1,
        }
    }

    
    pub fn sample_report_with_id(report_id: i32) -> Report {
        let new_report = sample_report();
        Report {
            reportid: report_id,
            createdat: Some(new_report.createdat.unwrap_or_else(|| Local::now().naive_local())),
            updatedat: new_report.updatedat,
            reporterfullname: new_report.reporterfullname,
            reporterphonenum: new_report.reporterphonenum,
            reporteraddress: new_report.reporteraddress,
            reporterrelationship: new_report.reporterrelationship,
            incidentlocation: new_report.incidentlocation,
            incidenttime: new_report.incidenttime,
            incidentdescription: new_report.incidentdescription,
            incidentvictimneeds: new_report.incidentvictimneeds,
            incidentproof: new_report.incidentproof,
            victimfullname: new_report.victimfullname,
            victimnik: new_report.victimnik,
            victimemail: new_report.victimemail,
            victimaddress: new_report.victimaddress,
            victimphonenum: new_report.victimphonenum,
            victimoccupation: new_report.victimoccupation,
            victimsex: new_report.victimsex,
            victimdateofbirth: new_report.victimdateofbirth,
            victimplaceofbirth: new_report.victimplaceofbirth,
            victimeducationlevel: new_report.victimeducationlevel,
            victimmarriagestatus: new_report.victimmarriagestatus,
            accusedfullname: new_report.accusedfullname,
            accusedaddress: new_report.accusedaddress,
            accusedphonenum: new_report.accusedphonenum,
            accusedoccupation: new_report.accusedoccupation,
            accusedsex: new_report.accusedsex,
            accusedrelationship: new_report.accusedrelationship,
            authority: new_report.authority,
            reporterid: new_report.reporterid,
        }
    }

    
    pub fn invalid_report_data() -> NewReport {
        let mut report = sample_report();
        
        report.victimemail = Some("not-a-valid-email".to_string());
        
        report.victimnik = Some("12345".to_string());
        report
    }

    
    pub fn malicious_report_data() -> NewReport {
        let mut report = sample_report();
        report.incidentdescription = Some("<script>alert('XSS')</script>Malicious description".to_string());
        report.victimfullname = "<img src=x onerror=alert('XSS')>Victim".to_string();
        report
    }
}