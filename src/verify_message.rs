// use axum::{
//     http::StatusCode,
//     response::{IntoResponse, Json},
// };

// use serde::{Deserialize, Serialize};

// #[derive(Deserialize, Serialize)]
// pub struct ErrorResponse {
//     pub success: bool,
//     pub error: String,
// }
// #[derive(Debug, Serialize, Deserialize)]
// pub struct SignatureVerifyRequest {
//     pub message: String,
//     pub signature: String,
//     pub pubkey: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct SignatureVerificationResponse {
//     pub success: bool,
//     pub data: SignatureData,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct SignatureData {
//     pub valid: bool,
//     pub message: String,
//     pub pubkey: String,
// }

// pub async fn verify_message(
//     Json(payload): Json<SignatureVerificationResponse>,
// ) -> impl IntoResponse {
//     if payload.data.message != "Hello, Solana!" {
//         return (
//             StatusCode::BAD_REQUEST,
//             Json(ErrorResponse {
//                 success: false,
//                 error: "Invalid message content".to_string(),
//             }),
//         );
//     }

//     // Validate base64 signature
//     if bs64::decode(&payload.signature).is_err() {
//         return (
//             StatusCode::BAD_REQUEST,
//             Json(ErrorResponse {
//                 success: false,
//                 error: "Invalid base64 signature".to_string(),
//             }),
//         );
//     }

//     // Validate base58 public key
//     if bs58::decode(&payload.pubkey).into_vec().is_err() {
//         return (
//             StatusCode::BAD_REQUEST,
//             Json(ErrorResponse {
//                 success: false,
//                 error: "Invalid base58 public key".to_string(),
//             }),
//         );
//     }

//     // If all good
//     (
//         StatusCode::OK,
//         Json(SuccessResponse {
//             success: true,
//             data: VerifyData {
//                 valid: true,
//                 message: payload.message,
//                 pubkey: payload.pubkey,
//             },
//         }),
//     )
// }
