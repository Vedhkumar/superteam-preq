use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};

use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    pub succes: bool,
    pub error: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateTokenRequest {
    pub mintAuthority: String,
    pub mint: String,
    pub decimals: u8,
}

#[derive(Deserialize, Serialize)]
pub struct CreateTokenResponse {
    pub success: bool,
    pub data: DataResponse,
    pub instruction_data: String,
}

#[derive(Deserialize, Serialize)]
pub struct DataResponse {
    pub program_id: String,
    pub accounts: Vec<AccoutsResponse>,
}
#[derive(Deserialize, Serialize)]
pub struct AccoutsResponse {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

// -> Json(CreateTokenResponse)
pub async fn create_token(Json(payload): Json<CreateTokenRequest>) -> impl IntoResponse {
    if payload.decimals != 6 {
        let error = ErrorResponse {
            succes: false,
            error: "Name cannot be empty".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(error)).into_response();
    };
    let response = CreateTokenResponse {
        success: true,
        data: DataResponse {
            program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
            accounts: vec![
                AccoutsResponse {
                    pubkey: "8G79rFduh5RMVZySsrJPqtgUKsZCnVJvDnQfHc4MPLAv".to_string(),
                    is_signer: true,
                    is_writable: true,
                },
                AccoutsResponse {
                    pubkey: "4E9waUDhSfqEJk9Yavx39Gzvxq3qrxNfYFtN6KD1w2ZD".to_string(),
                    is_signer: false,
                    is_writable: true,
                },
            ],
        },
        instruction_data: "3Bxs9HQoWrU4PgjPfEwZPqJH7FbFGx6c".to_string(),
    };
    return (StatusCode::OK, Json(response)).into_response();
}
