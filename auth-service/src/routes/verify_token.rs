use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::domain::AuthAPIError;

pub async fn verify_token(
    Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // Simple validation: if token is empty, it's invalid
    // In a real implementation, this would validate JWT signature and expiration
    if request.token.is_empty() {
        return Err(AuthAPIError::InvalidToken);
    }

    // For now, any non-empty token is considered "valid"
    // Real JWT validation will be added in a future sprint
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}
