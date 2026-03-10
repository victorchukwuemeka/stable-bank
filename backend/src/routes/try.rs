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
use uuid::Uuid;
use chrono::Utc;
use crate::AppState;

#[derive(Deserialize)]
pub struct DepositRequest {
    pub from_pubkey: String,
    pub amount_sol: f64,
}

#[derive(Serialize)]
pub struct DepositResponse {
    pub success: bool,
    pub message: String,
    pub signature: Option<String>,
}

pub async fn handle_deposit(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DepositRequest>,
) -> (StatusCode, Json<DepositResponse>) {
    let client = RpcClient::new(state.rpc_url.clone());

    let protocol_keypair = match read_keypair_file(&state.wallet_path) {
        Ok(k) => k,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DepositResponse {
                    success: false,
                    message: format!("Failed to load protocol wallet: {}", e),
                    signature: None,
                }),
            )
        }
    };

    let from_pubkey = match Pubkey::from_str(&payload.from_pubkey) {
        Ok(p) => p,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(DepositResponse {
                    success: false,
                    message: "Invalid public key".to_string(),
                    signature: None,
                }),
            )
        }
    };

    let lamports = (payload.amount_sol * 1_000_000_000.0) as u64;

    let instruction = system_instruction::transfer(
        &from_pubkey,
        &protocol_keypair.pubkey(),
        lamports,
    );

    let recent_blockhash = match client.get_latest_blockhash() {
        Ok(bh) => bh,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DepositResponse {
                    success: false,
                    message: format!("Failed to get blockhash: {}", e),
                    signature: None,
                }),
            )
        }
    };

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&from_pubkey),
        &[&protocol_keypair],
        recent_blockhash,
    );

    match client.send_and_confirm_transaction(&transaction) {
        Ok(sig) => {
            let now = Utc::now().to_rfc3339();
            let id = Uuid::new_v4().to_string();

            // save deposit record
            let _ = sqlx::query(
                "INSERT INTO deposits (id, pubkey, amount_sol, signature, created_at)
                 VALUES (?, ?, ?, ?, ?)"
            )
            .bind(&id)
            .bind(&payload.from_pubkey)
            .bind(payload.amount_sol)
            .bind(sig.to_string())
            .bind(&now)
            .execute(&state.db)
            .await;

            // upsert user record
            let _ = sqlx::query(
                "INSERT INTO users (pubkey, total_deposited, total_withdrawn, created_at)
                 VALUES (?, ?, 0, ?)
                 ON CONFLICT(pubkey) DO UPDATE SET
                 total_deposited = total_deposited + excluded.total_deposited"
            )
            .bind(&payload.from_pubkey)
            .bind(payload.amount_sol)
            .bind(&now)
            .execute(&state.db)
            .await;

            (
                StatusCode::OK,
                Json(DepositResponse {
                    success: true,
                    message: format!("Deposit of {} SOL successful", payload.amount_sol),
                    signature: Some(sig.to_string()),
                }),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(DepositResponse {
                success: false,
                message: format!("Transaction failed: {}", e),
                signature: None,
            }),
        ),
    }
}