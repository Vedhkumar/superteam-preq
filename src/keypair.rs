use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ErrorResponse {
    pub succes: bool,
    pub error: String,
}

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

pub async fn generate_keypair() -> Json<KeypairResponse> {
    return Json::from(KeypairResponse {
        success: true,
        data: Data {
            pubkey: "56nFYBxw8SHShJWkDDx1cpJKoyc1TdPjKfkP7eec2mGC".to_string(),
            secret: "3RzfngYfn639kZeKcYtA4EQcG7fi2XsFkfaiuzjbn2FfZEZtpe1Sai8FqT3LNEZ1VKo6pAqqDqYgWHuw4X5UaJMN".to_string(),
        },
    });
}
