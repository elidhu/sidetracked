use axum::{http::StatusCode, response::IntoResponse};

// @<handlerhealth_check
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
// >@
