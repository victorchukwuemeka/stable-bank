use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{read_keypair_file, Signer},
    stake,
    transaction::Transaction,
};
use std::str::FromStr;
use std::sync::Arc;
use chrono::Utc;
use crate::AppState;

#[derive(Deserialize)]
pub struct WithdrawRequest {
    pub pubkey: String,
}

#[derive(Deserialize)]
pub struct ClaimRequest {
    pub pubkey: String,
    pub stake_account: String,
}

#[derive(Serialize)]
pub struct WithdrawResponse {
    pub success: bool,
    pub message: String,
    pub signature: Option<String>,
}

// step 1 — deactivate all user stake accounts
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

    // fetch all active stake accounts for this user from db
    let rows = match sqlx::query_as::<_, (String, f64)>(
        "SELECT stake_account, amount_sol FROM stake_accounts WHERE pubkey = ?"
    )
    .bind(&payload.pubkey)
    .fetch_all(&state.db)
    .await {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(WithdrawResponse {
                    success: false,
                    message: format!("Failed to fetch stake accounts: {}", e),
                    signature: None,
                }),
            )
        }
    };

    if rows.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(WithdrawResponse {
                success: false,
                message: "No active stake accounts found for this user".to_string(),
                signature: None,
            }),
        );
    }

    let mut last_sig = String::new();

    for (stake_account_str, _) in &rows {
        let stake_pubkey = match Pubkey::from_str(stake_account_str) {
            Ok(p) => p,
            Err(_) => continue,
        };

        let deactivate_ix = stake::instruction::deactivate_stake(
            &stake_pubkey,
            &protocol_keypair.pubkey(),
        );

        let recent_blockhash = match client.get_latest_blockhash() {
            Ok(bh) => bh,
            Err(_) => continue,
        };

        let transaction = Transaction::new_signed_with_payer(
            &[deactivate_ix],
            Some(&protocol_keypair.pubkey()),
            &[&protocol_keypair],
            recent_blockhash,
        );

        match client.send_and_confirm_transaction(&transaction) {
            Ok(sig) => {
                last_sig = sig.to_string();
                println!("Deactivated stake account: {}", stake_account_str);

                // mark as deactivating in db
                let _ = sqlx::query(
                    "UPDATE stake_accounts SET signature = ? WHERE stake_account = ?"
                )
                .bind(&last_sig)
                .bind(stake_account_str)
                .execute(&state.db)
                .await;
            }
            Err(e) => {
                println!("Failed to deactivate {}: {}", stake_account_str, e);
            }
        }
    }

    (
        StatusCode::OK,
        Json(WithdrawResponse {
            success: true,
            message: "Stake deactivated. Wait one epoch then call /withdraw/claim".to_string(),
            signature: Some(last_sig),
        }),
    )
}

// step 2 — claim after cooldown
pub async fn handle_claim(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ClaimRequest>,
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

    let stake_pubkey = match Pubkey::from_str(&payload.stake_account) {
        Ok(p) => p,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(WithdrawResponse {
                    success: false,
                    message: "Invalid stake account address".to_string(),
                    signature: None,
                }),
            )
        }
    };

    let user_pubkey = match Pubkey::from_str(&payload.pubkey) {
        Ok(p) => p,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(WithdrawResponse {
                    success: false,
                    message: "Invalid user pubkey".to_string(),
                    signature: None,
                }),
            )
        }
    };

    // get stake account balance
    let stake_balance = match client.get_balance(&stake_pubkey) {
        Ok(b) => b,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(WithdrawResponse {
                    success: false,
                    message: format!("Failed to get stake balance: {}", e),
                    signature: None,
                }),
            )
        }
    };

    // withdraw from stake account to user wallet
    let withdraw_ix = stake::instruction::withdraw(
        &stake_pubkey,
        &protocol_keypair.pubkey(),
        &user_pubkey,
        stake_balance,
        None,
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
        &[withdraw_ix],
        Some(&protocol_keypair.pubkey()),
        &[&protocol_keypair],
        recent_blockhash,
    );

    match client.send_and_confirm_transaction(&transaction) {
        Ok(sig) => {
            // update db
            let _ = sqlx::query(
                "DELETE FROM stake_accounts WHERE stake_account = ?"
            )
            .bind(&payload.stake_account)
            .execute(&state.db)
            .await;

            let _ = sqlx::query(
                "UPDATE users SET total_withdrawn = total_withdrawn + ? WHERE pubkey = ?"
            )
            .bind(stake_balance as f64 / 1_000_000_000.0)
            .bind(&payload.pubkey)
            .execute(&state.db)
            .await;

            let now = Utc::now().to_rfc3339();
            let _ = sqlx::query(
                "INSERT INTO deposits (id, pubkey, amount_sol, signature, created_at)
                 VALUES (?, ?, ?, ?, ?)"
            )
            .bind(uuid::Uuid::new_v4().to_string())
            .bind(&payload.pubkey)
            .bind(-(stake_balance as f64 / 1_000_000_000.0))
            .bind(&sig.to_string())
            .bind(&now)
            .execute(&state.db)
            .await;

            (
                StatusCode::OK,
                Json(WithdrawResponse {
                    success: true,
                    message: format!(
                        "Withdrawn {} SOL to your wallet",
                        stake_balance as f64 / 1_000_000_000.0
                    ),
                    signature: Some(sig.to_string()),
                }),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(WithdrawResponse {
                success: false,
                message: format!("Claim failed - cooldown may not be complete yet: {}", e),
                signature: None,
            }),
        ),
    }
}