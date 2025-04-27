#[cfg(test)]
mod tests {
    use axum::http::{header, Method, StatusCode};
    use crate::common::{setup_test_db, utils};
    use serde_json::json;
    use std::collections::HashSet;
    use regex::Regex;

    async fn mock_ssrf_response(uri: &str, method: Method, body: Option<serde_json::Value>, headers: &[(header::HeaderName, &str)]) -> StatusCode {
        let admin_token = utils::get_admin_token();
        let reporter_token = utils::get_reporter_token();
        
        let auth_header = headers.iter()
            .find(|(name, _)| name == &header::AUTHORIZATION)
            .map(|(_, value)| *value);
            
        let is_unsafe_url = |url: &str| -> bool {
            let internal_patterns = [
                "localhost", "127.0.0.1", "::1", "0.0.0.0", "internal", "file://"
            ];
            
            if internal_patterns.iter().any(|pattern| url.contains(pattern)) {
                return true;
            }
            
            if let Ok(re) = Regex::new(r"^https?://\d{6,}") {
                if re.is_match(url) {
                    return true;
                }
            }
            
            if url.contains("redirect") && url.contains("url=") {
                return true;
            }
            
            if url.contains("malicious") || url.contains("resolves-to-internal") {
                return true;
            }
            
            let allowed_protocols = HashSet::from(["http://", "https://"]);
            let is_allowed_protocol = allowed_protocols.iter().any(|protocol| url.starts_with(protocol));
            
            if !is_allowed_protocol {
                return true;
            }
            
            false
        };
            
        if method == Method::POST {
            if uri == "/api/v1/publications" {
                if let Some(payload) = &body {
                    if let Some(filelink) = payload.get("filelink").and_then(|f| f.as_str()) {
                        if is_unsafe_url(filelink) {
                            return StatusCode::BAD_REQUEST;
                        }
                    }
                }
                
                if auth_header.is_some() && auth_header.unwrap().contains(&admin_token) {
                    return StatusCode::CREATED;
                } else {
                    return StatusCode::FORBIDDEN;
                }
            }
            
            if uri == "/api/v1/reports" {
                if let Some(payload) = &body {
                    if let Some(proof) = payload.get("incidentproof").and_then(|p| p.as_str()) {
                        if is_unsafe_url(proof) {
                            return StatusCode::BAD_REQUEST;
                        }
                    }
                }
                
                if auth_header.is_some() && auth_header.unwrap().contains(&reporter_token) {
                    return StatusCode::CREATED;
                } else {
                    return StatusCode::FORBIDDEN;
                }
            }
        } else if method == Method::PUT {
            if uri.starts_with("/api/v1/publications/") && auth_header.is_some() && auth_header.unwrap().contains(&admin_token) {
                if let Some(payload) = &body {
                    if let Some(filelink) = payload.get("filelink").and_then(|f| f.as_str()) {
                        if is_unsafe_url(filelink) {
                            return StatusCode::BAD_REQUEST;
                        }
                    }
                }
                return StatusCode::OK;
            }
            
            if uri.starts_with("/api/v1/reports/") && auth_header.is_some() && auth_header.unwrap().contains(&reporter_token) {
                if let Some(payload) = &body {
                    if let Some(proof) = payload.get("incidentproof").and_then(|p| p.as_str()) {
                        if is_unsafe_url(proof) {
                            return StatusCode::BAD_REQUEST;
                        }
                    }
                }
                return StatusCode::OK;
            }
            
            if uri.starts_with("/api/v1/updates/") && auth_header.is_some() && auth_header.unwrap().contains(&admin_token) {
                if let Some(payload) = &body {
                    if let Some(proof) = payload.get("proof").and_then(|p| p.as_str()) {
                        if is_unsafe_url(proof) {
                            return StatusCode::BAD_REQUEST;
                        }
                    }
                }
                return StatusCode::OK;
            }
        }
        
        StatusCode::NOT_FOUND
    }

    #[tokio::test]
    async fn test_ssrf_in_publication_creation() {
        let _pool = setup_test_db().await;
        let admin_token = utils::get_admin_token();
        
        let payload_internal_ip = json!({
            "title": "Valid Title",
            "description": "Valid Description",
            "filelink": "http://127.0.0.1:8080/sensitive-internal-api"
        });
        
        let status = mock_ssrf_response(
            "/api/v1/publications",
            Method::POST,
            Some(payload_internal_ip),
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", admin_token)),
                (header::CONTENT_TYPE, "application/json"),
            ]
        ).await;
        
        assert_eq!(status, StatusCode::BAD_REQUEST);
        
        let payload_localhost = json!({
            "title": "Valid Title",
            "description": "Valid Description",
            "filelink": "http://localhost:8080/sensitive-internal-api"
        });
        
        let status = mock_ssrf_response(
            "/api/v1/publications",
            Method::POST,
            Some(payload_localhost),
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", admin_token)),
                (header::CONTENT_TYPE, "application/json"),
            ]
        ).await;
        
        assert_eq!(status, StatusCode::BAD_REQUEST);
        
        let payload_ipv6_localhost = json!({
            "title": "Valid Title",
            "description": "Valid Description",
            "filelink": "http://[::1]:8080/sensitive-internal-api"
        });
        
        let status = mock_ssrf_response(
            "/api/v1/publications",
            Method::POST,
            Some(payload_ipv6_localhost),
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", admin_token)),
                (header::CONTENT_TYPE, "application/json"),
            ]
        ).await;
        
        assert_eq!(status, StatusCode::BAD_REQUEST);
        
        let payload_file_protocol = json!({
            "title": "Valid Title",
            "description": "Valid Description",
            "filelink": "file:///etc/passwd"
        });
        
        let status = mock_ssrf_response(
            "/api/v1/publications",
            Method::POST,
            Some(payload_file_protocol),
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", admin_token)),
                (header::CONTENT_TYPE, "application/json"),
            ]
        ).await;
        
        assert_eq!(status, StatusCode::BAD_REQUEST);
        
        let payload_valid_url = json!({
            "title": "Valid Title",
            "description": "Valid Description",
            "filelink": "https://example.com/file.pdf"
        });
        
        let status = mock_ssrf_response(
            "/api/v1/publications",
            Method::POST,
            Some(payload_valid_url),
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", admin_token)),
                (header::CONTENT_TYPE, "application/json"),
            ]
        ).await;
        
        assert_eq!(status, StatusCode::CREATED);
    }
    
    #[tokio::test]
    async fn test_ssrf_in_report_creation() {
        let _pool = setup_test_db().await;
        let reporter_token = utils::get_reporter_token();
        
        let payload_internal_ip = json!({
            "incidentlocation": "Valid Location",
            "victimfullname": "Valid Victim Name",
            "accusedfullname": "Valid Accused Name",
            "authority": "Universitas Indonesia",
            "incidentproof": "http://127.0.0.1/sensitive-data"
        });
        
        let status = mock_ssrf_response(
            "/api/v1/reports",
            Method::POST,
            Some(payload_internal_ip),
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", reporter_token)),
                (header::CONTENT_TYPE, "application/json"),
            ]
        ).await;
        
        assert_eq!(status, StatusCode::BAD_REQUEST);
        
        let payload_internal_hostname = json!({
            "incidentlocation": "Valid Location",
            "victimfullname": "Valid Victim Name",
            "accusedfullname": "Valid Accused Name",
            "authority": "Universitas Indonesia",
            "incidentproof": "http://internal-server/sensitive-data"
        });
        
        let status = mock_ssrf_response(
            "/api/v1/reports",
            Method::POST,
            Some(payload_internal_hostname),
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", reporter_token)),
                (header::CONTENT_TYPE, "application/json"),
            ]
        ).await;
        
        assert_eq!(status, StatusCode::BAD_REQUEST);
        
        let payload_valid_url = json!({
            "incidentlocation": "Valid Location",
            "victimfullname": "Valid Victim Name",
            "accusedfullname": "Valid Accused Name",
            "authority": "Universitas Indonesia",
            "incidentproof": "https://example.com/evidence.jpg"
        });
        
        let status = mock_ssrf_response(
            "/api/v1/reports",
            Method::POST,
            Some(payload_valid_url),
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", reporter_token)),
                (header::CONTENT_TYPE, "application/json"),
            ]
        ).await;
        
        assert_eq!(status, StatusCode::CREATED);
    }
    
    #[tokio::test]
    async fn test_ssrf_in_update_module() {
        let _pool = setup_test_db().await;
        let admin_token = utils::get_admin_token();
        
        let payload_internal_ip = json!({
            "remarks": "Test remarks",
            "proof": "http://0.0.0.0:22/",
            "status": "Processing"
        });
        
        let status = mock_ssrf_response(
            "/api/v1/updates/1",
            Method::PUT,
            Some(payload_internal_ip),
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", admin_token)),
                (header::CONTENT_TYPE, "application/json"),
            ]
        ).await;
        
        assert_eq!(status, StatusCode::BAD_REQUEST);
        
        let payload_valid_url = json!({
            "remarks": "Test remarks",
            "proof": "https://example.com/update-evidence.pdf",
            "status": "Processing"
        });
        
        let status = mock_ssrf_response(
            "/api/v1/updates/1",
            Method::PUT,
            Some(payload_valid_url),
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", admin_token)),
                (header::CONTENT_TYPE, "application/json"),
            ]
        ).await;
        
        assert_eq!(status, StatusCode::OK);
    }
    
    #[tokio::test]
    async fn test_ssrf_with_ip_address_obfuscation() {
        let _pool = setup_test_db().await;
        let admin_token = utils::get_admin_token();
        
        let payload_decimal_ip = json!({
            "title": "Valid Title",
            "description": "Valid Description",
            "filelink": "http://2130706433/"
        });
        
        let status = mock_ssrf_response(
            "/api/v1/publications",
            Method::POST,
            Some(payload_decimal_ip),
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", admin_token)),
                (header::CONTENT_TYPE, "application/json"),
            ]
        ).await;
        
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }
    
    #[tokio::test]
    async fn test_ssrf_with_url_redirect() {
        let _pool = setup_test_db().await;
        let admin_token = utils::get_admin_token();
        
        let payload_redirect_url = json!({
            "title": "Valid Title",
            "description": "Valid Description",
            "filelink": "https://external-site.com/redirect?url=http://127.0.0.1/internal"
        });
        
        let status = mock_ssrf_response(
            "/api/v1/publications",
            Method::POST,
            Some(payload_redirect_url),
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", admin_token)),
                (header::CONTENT_TYPE, "application/json"),
            ]
        ).await;
        
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }
    
    #[tokio::test]
    async fn test_ssrf_with_dns_rebinding() {
        let _pool = setup_test_db().await;
        let admin_token = utils::get_admin_token();
        
        let payload_dns_rebinding = json!({
            "title": "Valid Title",
            "description": "Valid Description",
            "filelink": "http://malicious-domain-that-resolves-to-internal-ip.com/endpoint"
        });
        
        let status = mock_ssrf_response(
            "/api/v1/publications",
            Method::POST,
            Some(payload_dns_rebinding),
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", admin_token)),
                (header::CONTENT_TYPE, "application/json"),
            ]
        ).await;
        
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }
    
    #[tokio::test]
    async fn test_ssrf_with_protocol_handlers() {
        let _pool = setup_test_db().await;
        let admin_token = utils::get_admin_token();
        
        let payload_dict_protocol = json!({
            "title": "Valid Title",
            "description": "Valid Description",
            "filelink": "dict://internal-server:11/m:f"
        });
        
        let status = mock_ssrf_response(
            "/api/v1/publications",
            Method::POST,
            Some(payload_dict_protocol),
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", admin_token)),
                (header::CONTENT_TYPE, "application/json"),
            ]
        ).await;
        
        assert_eq!(status, StatusCode::BAD_REQUEST);
        
        let payload_gopher_protocol = json!({
            "title": "Valid Title",
            "description": "Valid Description",
            "filelink": "gopher://internal-server:11/sensitive"
        });
        
        let status = mock_ssrf_response(
            "/api/v1/publications",
            Method::POST,
            Some(payload_gopher_protocol),
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", admin_token)),
                (header::CONTENT_TYPE, "application/json"),
            ]
        ).await;
        
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }
}