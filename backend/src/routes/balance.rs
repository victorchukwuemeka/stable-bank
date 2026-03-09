use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::Serialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use std::sync::Arc;
use crate::AppState;

#[derive(Serialize)]
pub struct BalanceResponse {
    pub pubkey: String,
    pub balance_sol: f64,
}

pub async fn handle_balance(
    Path(pubkey): Path<String>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<BalanceResponse>) {
    let client = RpcClient::new(state.rpc_url.clone());

    let pubkey = match Pubkey::from_str(&pubkey) {
        Ok(p) => p,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(BalanceResponse {
                    pubkey: pubkey.to_string(),
                    balance_sol: 0.0,
                }),
            )
        }
    };

    let lamports = client.get_balance(&pubkey).unwrap_or(0);
    let sol = lamports as f64 / 1_000_000_000.0;

    (
        StatusCode::OK,
        Json(BalanceResponse {
            pubkey: pubkey.to_string(),
            balance_sol: sol,
        }),
    )
}