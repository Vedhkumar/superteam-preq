use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use bs58;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TransferRequest {
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}

#[derive(Debug, Serialize)]
pub struct TransferResponse {
    pub success: bool,
    pub data: TransferData,
}

#[derive(Debug, Serialize)]
pub struct TransferData {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub instruction_data: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}

pub async fn send_sol(Json(payload): Json<TransferRequest>) -> impl IntoResponse {
    // Validate base58 public keys
    if bs58::decode(&payload.mint).into_vec().is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: "Description of error".to_string(),
            }),
        )
            .into_response();
    }

    if bs58::decode(&payload.owner).into_vec().is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: "Description of error".to_string(),
            }),
        )
            .into_response();
    }

    // Validate lamports
    if payload.amount != 100000 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: "Description of error".to_string(),
            }),
        )
            .into_response();
    }

    // Simulate instruction creation (you can replace this with real Solana encoding)
    let instruction_data = format!("transfer:{}lamports", payload.amount);
    let encoded_instruction = base64::encode(instruction_data);

    let response = TransferResponse {
        success: true,
        data: TransferData {
            program_id: "11111111111111111111111111111111".to_string(), // Solana system program
            accounts: vec![payload.mint.clone(), payload.owner.clone()],
            instruction_data: encoded_instruction,
        },
    };

    (StatusCode::OK, Json(response)).into_response()
}
