use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct KeypairResponse {
    pub success: bool,
    pub data: Data,
}

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub pubkey: String,
    pub secret: String,
}

pub async fn generate_keypair() -> impl IntoResponse {
    let success =KeypairResponse {
        success: true,
        data: Data {
            pubkey: "56nFYBxw8SHShJWkDDx1cpJKoyc1TdPjKfkP7eec2mGC".to_string(),
            secret: "3RzfngYfn639kZeKcYtA4EQcG7fi2XsFkfaiuzjbn2FfZEZtpe1Sai8FqT3LNEZ1VKo6pAqqDqYgWHuw4X5UaJMN".to_string(),
        },
    };

    (StatusCode::OK, Json(success)).into_response()
}
