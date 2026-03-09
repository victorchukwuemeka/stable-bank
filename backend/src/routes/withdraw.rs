use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{read_keypair_file, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::str::FromStr;
use std::sync::Arc;
use crate::AppState;

#[derive(Deserialize)]
pub struct WithdrawRequest {
    pub to_pubkey: String,
    pub amount_sol: f64,
}

#[derive(Serialize)]
pub struct WithdrawResponse {
    pub success: bool,
    pub message: String,
    pub signature: Option<String>,
}

pub async fn handle_withdraw(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<WithdrawRequest>,
) -> (StatusCode, Json<WithdrawResponse>) {
    let client = RpcClient::new(state.rpc_url.clone());

    let protocol_keypair = match read_keypair_file(&state.wallet_path) {
        Ok(k) => k,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(WithdrawResponse {
                    success: false,
                    message: format!("Failed to load protocol wallet: {}", e),
                    signature: None,
                }),
            )
        }
    };

    let to_pubkey = match Pubkey::from_str(&payload.to_pubkey) {
        Ok(p) => p,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(WithdrawResponse {
                    success: false,
                    message: "Invalid public key".to_string(),
                    signature: None,
                }),
            )
        }
    };

    let lamports = (payload.amount_sol * 1_000_000_000.0) as u64;

    // check protocol has enough balance
    let protocol_balance = match client.get_balance(&protocol_keypair.pubkey()) {
        Ok(b) => b,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(WithdrawResponse {
                    success: false,
                    message: format!("Failed to get protocol balance: {}", e),
                    signature: None,
                }),
            )
        }
    };

    if lamports > protocol_balance {
        return (
            StatusCode::BAD_REQUEST,
            Json(WithdrawResponse {
                success: false,
                message: format!(
                    "Insufficient protocol balance. Available: {} SOL",
                    protocol_balance as f64 / 1_000_000_000.0
                ),
                signature: None,
            }),
        );
    }

    let instruction = system_instruction::transfer(
        &protocol_keypair.pubkey(),
        &to_pubkey,
        lamports,
    );

    let recent_blockhash = match client.get_latest_blockhash() {
        Ok(bh) => bh,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(WithdrawResponse {
                    success: false,
                    message: format!("Failed to get blockhash: {}", e),
                    signature: None,
                }),
            )
        }
    };

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&protocol_keypair.pubkey()),
        &[&protocol_keypair],
        recent_blockhash,
    );

    match client.send_and_confirm_transaction(&transaction) {
        Ok(sig) => (
            StatusCode::OK,
            Json(WithdrawResponse {
                success: true,
                message: format!("Withdrawal of {} SOL successful", payload.amount_sol),
                signature: Some(sig.to_string()),
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(WithdrawResponse {
                success: false,
                message: format!("Transaction failed: {}", e),
                signature: None,
            }),
        ),
    }
}