use axum::{
    Router,
    routing::{get, post},
};
mod create_token;
mod keypair;
mod sign_message;

use create_token::create_token;
use keypair::generate_keypair;
use sign_message::sign_message;
#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/keypair", get(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/message/sign", post(sign_message));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
