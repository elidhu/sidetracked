use axum::{http::StatusCode, response::IntoResponse, Json};
use jwt_authorizer::JwtClaims;

use super::auth::Claims;

// @<handlerhealth_check
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
// >@

// @<handlerprofile
pub async fn profile(JwtClaims(claims): JwtClaims<Claims>) -> impl IntoResponse {
    Json(claims)
}
// >@
