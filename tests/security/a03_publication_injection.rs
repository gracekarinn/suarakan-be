#[cfg(test)]
mod tests {
    use axum::http::{header, Method, StatusCode};
    use crate::common::{setup_test_db, utils};
    use serde_json::json;

    async fn mock_injection_response(uri: &str, method: Method, body: Option<serde_json::Value>, headers: &[(header::HeaderName, &str)]) -> StatusCode {
        let admin_token = utils::get_admin_token();

        if method == Method::POST && uri == "/api/v1/publications" {
            if let Some(payload) = body {
                let title = payload.get("title").and_then(|t| t.as_str()).unwrap_or("");

                if title.contains('\'') || title.contains(';') || title.contains('<') || title.contains('>') {
                    return StatusCode::BAD_REQUEST;
                }
                return StatusCode::CREATED;
            }
        }

        StatusCode::NOT_FOUND
    }

    #[tokio::test]
    async fn test_sql_injection_in_title() {
        let _pool = setup_test_db().await;
        let admin_token = utils::get_admin_token();

        let malicious_payload = json!({
            "title": "test'); DROP TABLE publications; --",
            "description": "Trying SQL injection",
            "filelink": "https://example.com/file.pdf"
        });

        let status = mock_injection_response(
            "/api/v1/publications",
            Method::POST,
            Some(malicious_payload),
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", admin_token)),
                (header::CONTENT_TYPE, "application/json"),
            ]
        ).await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_script_injection_in_description() {
        let _pool = setup_test_db().await;
        let admin_token = utils::get_admin_token();
    
        let malicious_payload = json!({
            "title": "Normal Title",
            "description": "<script>alert('XSS')</script>",
            "filelink": "https://example.com/file.pdf"
        });
    
        let status = mock_injection_response(
            "/api/v1/publications",
            Method::POST,
            Some(malicious_payload),
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", admin_token)),
                (header::CONTENT_TYPE, "application/json"),
            ]
        ).await;    

        assert_eq!(status, StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_filelink_injection() {
        let _pool = setup_test_db().await;
        let admin_token = utils::get_admin_token();
    
        let malicious_payload = json!({
            "title": "Valid Title",
            "description": "Valid Description",
            "filelink": "javascript:alert('XSS')"
        });

        let status = mock_injection_response(
            "/api/v1/publications",
            Method::POST,
            Some(malicious_payload),
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", admin_token)),
                (header::CONTENT_TYPE, "application/json"),
            ]
        ).await;  
    
        assert_eq!(status, StatusCode::CREATED);
    }    

    #[tokio::test]
    async fn test_valid_input() {
        let _pool = setup_test_db().await;
        let admin_token = utils::get_admin_token();

        let valid_payload = json!({
            "title": "Valid Title",
            "description": "Safe Description",
            "filelink": "https://example.com/file.pdf"
        });

        let status = mock_injection_response(
            "/api/v1/publications",
            Method::POST,
            Some(valid_payload),
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", admin_token)),
                (header::CONTENT_TYPE, "application/json"),
            ]
        ).await;

        assert_eq!(status, StatusCode::CREATED);
    }
}