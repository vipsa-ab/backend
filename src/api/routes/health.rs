use axum::Json;

use crate::api::dtos::HealthResponse;

pub async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn health_returns_200() {
        let response = axum::Router::new()
            .route("/health", axum::routing::get(health_handler))
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn health_returns_ok_status() {
        let response = axum::Router::new()
            .route("/health", axum::routing::get(health_handler))
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: HealthResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(json.status, "ok");
    }

    #[tokio::test]
    async fn health_returns_version() {
        let response = axum::Router::new()
            .route("/health", axum::routing::get(health_handler))
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: HealthResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(json.version, env!("CARGO_PKG_VERSION"));
    }

    #[tokio::test]
    async fn health_returns_valid_timestamp() {
        let response = axum::Router::new()
            .route("/health", axum::routing::get(health_handler))
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: HealthResponse = serde_json::from_slice(&body).unwrap();
        assert!(chrono::DateTime::parse_from_rfc3339(&json.timestamp).is_ok());
    }

    #[tokio::test]
    async fn health_returns_json_content_type() {
        let response = axum::Router::new()
            .route("/health", axum::routing::get(health_handler))
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let content_type = response
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap();
        assert!(content_type.contains("application/json"));
    }
}
