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
pub struct Request {
    message: String,
    secret: String,
}
#[derive(Deserialize, Serialize)]

pub struct Response {
    success: bool,
    data: DataResponse,
}

#[derive(Deserialize, Serialize)]
pub struct DataResponse {
    signature: String,
    pubkey: String,
    message: String,
}
pub async fn sign_message(Json(payload): Json<Request>) -> impl IntoResponse {
    if bs58::decode(&payload.secret).into_vec().is_err() {
        let error = ErrorResponse {
            succes: false,
            error: "Description of error".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(error)).into_response();
    }
    let res = Response {
        success: true,
        data: DataResponse {
            signature: "3xsPnrh3P7QjjwT1qRHTzLa7f3aWBCdyaAYh3HPmshKWVmFqMQjUMdL66Z3JhAFqG9Rw4uZnHFybZug3XGRXpNQ7"
                .to_string(),
            pubkey: "8G79rFduh5RMVZySsrJPqtgUKsZCnVJvDnQfHc4MPLAv".to_string(),
            message: "Hello, Solana!".to_string(),
        },
    };
    return (StatusCode::OK, Json(res)).into_response();
}
