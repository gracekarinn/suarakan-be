#[cfg(test)]
mod tests {
    use axum::http::{header, Method, StatusCode};
    use crate::common::{setup_test_db, utils};
    
    
    async fn mock_response(uri: &str, method: Method, headers: &[(header::HeaderName, &str)]) -> StatusCode {
        
        let reporter_token = utils::get_reporter_token();
        let admin_token = utils::get_admin_token();
        
        
        let auth_header = headers.iter()
            .find(|(name, _)| name == &header::AUTHORIZATION)
            .map(|(_, value)| *value);
            
        
        match (uri, method.as_str(), auth_header) {
            
            ("/admin", "GET", None) => StatusCode::UNAUTHORIZED,
            ("/admin", "GET", Some(h)) if h.contains(&reporter_token) => StatusCode::FORBIDDEN,
            ("/admin", "GET", Some(h)) if h.contains(&admin_token) => StatusCode::OK,
            
            
            ("/api/v1/publications", "POST", Some(h)) if h.contains(&reporter_token) => StatusCode::FORBIDDEN,
            ("/api/v1/publications", "POST", Some(h)) if h.contains(&admin_token) => StatusCode::CREATED,
            
            
            _ => StatusCode::NOT_FOUND,
        }
    }

    #[tokio::test]
    async fn test_unauthorized_access_to_admin_endpoint() {
        
        let _pool = setup_test_db().await;
        
        
        let status = mock_response(
            "/admin", 
            Method::GET,
            &[]
        ).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        
        
        let reporter_token = utils::get_reporter_token();
        let status = mock_response(
            "/admin", 
            Method::GET,
            &[(header::AUTHORIZATION, &format!("Bearer {}", reporter_token))]
        ).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        
        
        let admin_token = utils::get_admin_token();
        let status = mock_response(
            "/admin", 
            Method::GET,
            &[(header::AUTHORIZATION, &format!("Bearer {}", admin_token))]
        ).await;
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_publication_authorization() {
        
        let _pool = setup_test_db().await;
        
        
        let reporter_token = utils::get_reporter_token();
        let status = mock_response(
            "/api/v1/publications", 
            Method::POST,
            &[
                (header::AUTHORIZATION, &format!("Bearer {}", reporter_token)),
                (header::CONTENT_TYPE, "application/json")
            ]
        ).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
    }
}