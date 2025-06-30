use axum::{Router, routing::post};
mod create_token;
mod keypair;
mod mint_token;
mod sign_message;
mod verify_message;

use create_token::create_token;
use keypair::generate_keypair;
use mint_token::mint_token;
use sign_message::sign_message;
// use verify_message::verify_message;
#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/message/sign", post(sign_message))
        .route("/token/mint", post(mint_token));
    // .route("/message/verify", post(verify_message));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
