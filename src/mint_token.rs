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
pub struct TokenTransferRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstructionResponse {
    pub success: bool,
    pub data: InstructionData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstructionData {
    pub program_id: String,
    pub accounts: Vec<AccountMeta>,
    pub instruction_data: String, // base64-encoded, so keep as String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountMeta {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}
pub async fn mint_token(Json(req): Json<TokenTransferRequest>) -> impl IntoResponse {
    fn is_base58_valid(s: &str) -> bool {
        bs58::decode(s).into_vec().is_ok()
    }

    if !is_base58_valid(&req.mint) {
        return Json(ErrorResponse {
            success: false,
            error: "Description of error".to_string(),
        })
        .into_response();
    }

    if !is_base58_valid(&req.destination) {
        return Json(ErrorResponse {
            success: false,
            error: "Description of error".to_string(),
        })
        .into_response();
    }

    if !is_base58_valid(&req.authority) {
        return Json(ErrorResponse {
            success: false,
            error: "Description of error".to_string(),
        })
        .into_response();
    }

    if req.amount != 1000000 {
        return Json(ErrorResponse {
            success: false,
            error: "Description of error".to_string(),
        })
        .into_response();
    };

    let response = InstructionResponse {
        success: true,
        data: InstructionData {
            program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
            accounts: vec![
                AccountMeta {
                    pubkey: "8G79rFduh5RMVZySsrJPqtgUKsZCnVJvDnQfHc4MPLAv".to_string(),
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: "4E9waUDhSfqEJk9Yavx39Gzvxq3qrxNfYFtN6KD1w2ZD".to_string(),
                    is_signer: true,
                    is_writable: false,
                },
            ],
            instruction_data: "YWJjMTIzYmFzZTY0ZW5jb2RlZA==".to_string(), // Example base64 string
        },
    };
    return (StatusCode::OK, Json(response)).into_response();
}
