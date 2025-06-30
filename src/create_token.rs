// use axum::Json;
// use serde::de;
// use solana_sdk::pubkey::Pubkey;

// #[derive(serde::Deserialize, serde::Serialize)]
// struct CreateTokenRequest {
//     mintAuthority: String,
//     mint: String,
//     decimals: u8,
// }

// struct CreateTokenResponse {
//     success: bool,
//     data: DataResponse,
//     instruction_data: String,
// }

// struct DataResponse {
//     program_id: String,
//     accounts: AccoutsResponse,
// }
// struct AccoutsResponse {
//     pubkey: String,
//     is_signer: bool,
//     is_writable: bool,
// }

// // -> Json(CreateTokenResponse)
// async fn create_token(Json(req): Json<CreateTokenRequest>) {
//     let mint_authority = Pubkey::from(req.mintAuthority);
//     let mint = Pubkey::from(req.mint);
//     let decimals = req.decimals;
// }
