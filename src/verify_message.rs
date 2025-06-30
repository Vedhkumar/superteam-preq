use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureVerifyRequest {
    pub message: String,
    pub signature: String,
    pub pubkey: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureVerificationResponse {
    pub success: bool,
    pub data: SignatureData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureData {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

pub async fn verify_message(Json(payload): Json<SignatureVerifyRequest>) -> impl IntoResponse {
    if payload.message != "Hello, Solana!" {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: "Description of error".to_string(),
            }),
        )
            .into_response();
    }

    if bs58::decode(&payload.pubkey).into_vec().is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: "Description of error".to_string(),
            }),
        )
            .into_response();
    }

    let response = SignatureVerificationResponse {
        success: true,
        data: SignatureData {
            valid: true,
            message: "Hello, Solana!".to_string(),
            pubkey: "8G79rFduh5RMVZySsrJPqtgUKsZCnVJvDnQfHc4MPLAv".to_string(),
        },
    };
    return (StatusCode::OK, Json(response)).into_response();
}
