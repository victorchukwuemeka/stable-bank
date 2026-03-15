use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use solana_sdk::signature::{read_keypair_file, Signer};
use std::sync::Arc;
use crate::AppState;

#[derive(Serialize)]
pub struct ProtocolConfig {
    pub protocol_wallet: String,
    pub network: String,
}

pub async fn handle_config(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<ProtocolConfig>) {
    let keypair = read_keypair_file(&state.wallet_path)
        .expect("Failed to load keypair");

    (
        StatusCode::OK,
        Json(ProtocolConfig {
            protocol_wallet: keypair.pubkey().to_string(),
            network: "devnet".to_string(),
        }),
    )
}