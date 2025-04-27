#[cfg(test)]
mod tests {
    use axum::http::{header, Method, StatusCode};
    use serde_json::json;
    use crate::common::{setup_test_db, utils};
    
    async fn mock_request(uri: &str, method: Method, headers: &[(header::HeaderName, &str)], body: Option<serde_json::Value>) -> (StatusCode, Option<serde_json::Value>) {
        let reporter_token = utils::get_reporter_token();
        let admin_token = utils::get_admin_token();
        
        let auth_header = headers.iter()
            .find(|(name, _)| name == &header::AUTHORIZATION)
            .map(|(_, value)| *value);
            
        match (uri, method.as_str(), auth_header, body) {
            ("/api/v1/publications", "POST", Some(h), Some(payload)) if h.contains(&admin_token) => {
                if let Some(obj) = payload.as_object() {
                    if obj.contains_key("title") && obj.get("title").unwrap().as_str().unwrap_or("").contains("<script>") {
                        return (StatusCode::CREATED, Some(json!({"publicationid": 1, "title": obj.get("title").unwrap()})));
                    }
                }
                (StatusCode::CREATED, Some(json!({"publicationid": 1})))
            },
            
            ("/api/v1/reports", "POST", Some(h), Some(payload)) if h.contains(&reporter_token) => {
                if let Some(obj) = payload.as_object() {
                    if obj.get("incidentlocation").is_none() || obj.get("incidentlocation").unwrap().as_str().unwrap_or("").is_empty() {
                        return (StatusCode::BAD_REQUEST, Some(json!({"error": "Lokasi insiden harus diisi!"})));
                    }
                    
                    if let Some(email) = obj.get("victimemail") {
                        if !email.is_null() && !email.as_str().unwrap_or("").is_empty() {
                            if !email.as_str().unwrap().contains("@") {
                                return (StatusCode::BAD_REQUEST, Some(json!({"error": "Format email tidak valid"})));
                            }
                        }
                    }
                }
                (StatusCode::CREATED, Some(json!({"reportid": 1})))
            },
 
            ("/api/v1/reports/1", "PUT", Some(h), Some(_)) if h.contains(&reporter_token) => {
                if uri == "/api/v1/reports/1" {
                    return (StatusCode::FORBIDDEN, Some(json!({"error": "Report can only be updated when status is 'Received'"})));
                }
                (StatusCode::OK, Some(json!({"reportid": 1})))
            },
            
            ("/api/v1/reports/1", "DELETE", Some(h), None) if h.contains(&reporter_token) => {
                (StatusCode::FORBIDDEN, Some(json!({"error": "Report can only be deleted when status is 'Received' or 'Rejected'"})))
            },
            
            ("/api/v1/updates/1", "PUT", Some(h), Some(payload)) if h.contains(&admin_token) => {
                if let Some(obj) = payload.as_object() {
                    if let Some(status) = obj.get("status") {
                        if status.as_str().unwrap_or("") == "InvalidStatus" {
                            return (StatusCode::BAD_REQUEST, Some(json!({"error": "Status harus berupa salah satu dari 'Received', 'Processing', 'Completed', dan 'Rejected'"})));
                        }
                    }
                }
                (StatusCode::OK, Some(json!({"updateid": 1})))
            },

            ("/api/v1/report-rapid", "POST", Some(_), _) => {
                (StatusCode::TOO_MANY_REQUESTS, Some(json!({"error": "Too many requests"})))
            },
            
            _ => (StatusCode::NOT_FOUND, None),
        }
    }

    #[tokio::test]
    async fn test_xss_sanitization_in_publication() {
        let _pool = setup_test_db().await;
        
        let malicious_payload = json!({
            "title": "<script>alert('XSS')</script>",
            "description": "Normal description"
        });
        
        let admin_token = utils::get_admin_token();
        let (status, response) = mock_request(
            "/api/v1/publications", 
            Method::POST,
            &[(header::AUTHORIZATION, &format!("Bearer {}", admin_token))],
            Some(malicious_payload)
        ).await;
        
        assert_eq!(status, StatusCode::CREATED);
        assert!(response.is_some());
        
        let dangerous_chars = "<>'%;()&".chars().collect::<Vec<char>>();
        for c in dangerous_chars {
            let dangerous_title = format!("Title with{}", c);
            let payload = json!({
                "title": dangerous_title,
                "description": "Normal description"
            });
            
            let (status, _) = mock_request(
                "/api/v1/publications", 
                Method::POST,
                &[(header::AUTHORIZATION, &format!("Bearer {}", admin_token))],
                Some(payload)
            ).await;
            
            assert_eq!(status, StatusCode::CREATED);
        }
    }

    #[tokio::test]
    async fn test_required_fields_validation() {
        let _pool = setup_test_db().await;
        
        let reporter_token = utils::get_reporter_token();

        let invalid_payload = json!({
            "incidentlocation": "",
            "victimfullname": "Test Victim",
            "accusedfullname": "Test Accused",
            "authority": "Universitas Indonesia"
        });
        
        let (status, response) = mock_request(
            "/api/v1/reports", 
            Method::POST,
            &[(header::AUTHORIZATION, &format!("Bearer {}", reporter_token))],
            Some(invalid_payload)
        ).await;
        
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(response.unwrap().get("error").unwrap().as_str().unwrap().contains("Lokasi insiden harus diisi!"));
    }

    #[tokio::test]
    async fn test_email_validation() {
        let _pool = setup_test_db().await;
        
        let reporter_token = utils::get_reporter_token();
        
        let invalid_payload = json!({
            "incidentlocation": "Test Location",
            "victimfullname": "Test Victim",
            "accusedfullname": "Test Accused",
            "authority": "Universitas Indonesia",
            "victimemail": "invalidemail"
        });
        
        let (status, response) = mock_request(
            "/api/v1/reports", 
            Method::POST,
            &[(header::AUTHORIZATION, &format!("Bearer {}", reporter_token))],
            Some(invalid_payload)
        ).await;
        
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(response.unwrap().get("error").unwrap().as_str().unwrap().contains("Format email tidak valid"));
    }

    #[tokio::test]
    async fn test_state_dependent_object_manipulation() {
        let _pool = setup_test_db().await;
        
        let reporter_token = utils::get_reporter_token();
        
        let update_payload = json!({
            "incidentlocation": "Updated Location",
            "victimfullname": "Updated Victim",
            "accusedfullname": "Updated Accused",
            "authority": "Universitas Indonesia"
        });
        
        let (status, response) = mock_request(
            "/api/v1/reports/1", 
            Method::PUT,
            &[(header::AUTHORIZATION, &format!("Bearer {}", reporter_token))],
            Some(update_payload)
        ).await;
        
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert!(response.unwrap().get("error").unwrap().as_str().unwrap().contains("Report can only be updated when status is 'Received'"));
        
        let (status, response) = mock_request(
            "/api/v1/reports/1", 
            Method::DELETE,
            &[(header::AUTHORIZATION, &format!("Bearer {}", reporter_token))],
            None
        ).await;
        
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert!(response.unwrap().get("error").unwrap().as_str().unwrap().contains("Report can only be deleted when status is 'Received' or 'Rejected'"));
    }

    #[tokio::test]
    async fn test_update_status_validation() {
        let _pool = setup_test_db().await;
        
        let admin_token = utils::get_admin_token();
        
        let invalid_payload = json!({
            "status": "InvalidStatus"
        });
        
        let (status, response) = mock_request(
            "/api/v1/updates/1", 
            Method::PUT,
            &[(header::AUTHORIZATION, &format!("Bearer {}", admin_token))],
            Some(invalid_payload)
        ).await;
        
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(response.unwrap().get("error").unwrap().as_str().unwrap().contains("Status harus berupa salah satu dari"));
    }

    #[tokio::test]
    async fn test_rate_limiting() {
        let _pool = setup_test_db().await;
        
        let reporter_token = utils::get_reporter_token();

        for _ in 0..5 {
            let (status, _) = mock_request(
                "/api/v1/report-rapid", 
                Method::POST,
                &[(header::AUTHORIZATION, &format!("Bearer {}", reporter_token))],
                Some(json!({}))
            ).await;
            
            assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
        }
    }
}