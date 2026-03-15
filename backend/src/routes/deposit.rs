use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::read_keypair_file;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use crate::AppState;
use crate::staking::stake_native;
//use solana_transaction_status::UiTransactionEncoding;

#[derive(Deserialize)]
pub struct DepositRequest {
    pub from_pubkey: String,
    pub amount_sol: f64,
    pub signature: String,
}

#[derive(Serialize)]
pub struct DepositResponse {
    pub success: bool,
    pub message: String,
}

pub async fn handle_deposit(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DepositRequest>,
) -> (StatusCode, Json<DepositResponse>) {
    let client = RpcClient::new(state.rpc_url.clone());

    // confirm the transaction actually landed on-chain
    println!("Checking transaction: {}", payload.signature);
    
    let sig = match payload.signature.parse() {
        Ok(s) => s,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(DepositResponse {
                    success: false,
                    message: "Invalid signature".to_string(),
                }),
            )
        }
    };
    
    /* 
    match client.confirm_transaction(&sig) {
        Ok(true) => {}
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(DepositResponse {
                    success: false,
                    message: "Transaction not confirmed on chain".to_string(),
                }),
            )
        }
    }
    
    */

    println!("Parsed signature: {}", sig);
    println!("Checking RPC: {}", state.rpc_url);
    
    let mut confirmed = false;

    for attempt  in 0..5 {

        println!("Attempt {} to find transaction...", attempt + 1);
        match client.get_transaction(
        &sig, 
        solana_transaction_status::UiTransactionEncoding::Json
        ){
            Ok(tx) =>{
                println!("tx found {:?}", tx.slot);
                confirmed = true;
                break;
            },
            Err(e)=> {
                 println!("Not found yet: {}", e);
                 tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
            }
        }
    }

    if !confirmed {
        return (
            StatusCode::BAD_REQUEST,
            Json(DepositResponse {
                success: false,
                message: "Transaction not found on chain after retries".to_string(),
            }),
        );
    }
    

    // record deposit in db
    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let _ = sqlx::query(
        "INSERT INTO deposits (id, pubkey, amount_sol, signature, created_at)
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&payload.from_pubkey)
    .bind(payload.amount_sol)
    .bind(&payload.signature)
    .bind(&now)
    .execute(&state.db)
    .await;

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

    // auto stake
    let protocol_keypair = match read_keypair_file(&state.wallet_path) {
        Ok(k) => k,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DepositResponse {
                    success: false,
                    message: format!("Failed to load protocol wallet: {}", e),
                }),
            )
        }
    };

    match stake_native(&state.rpc_url, &protocol_keypair, payload.amount_sol).await {
        Ok((stake_sig, stake_account)) => {
            println!("Auto staked natively. Stake signature: {}", stake_sig);
            let stake_id = Uuid::new_v4().to_string();
            let _ = sqlx::query(
                "INSERT INTO stake_accounts (id, pubkey, stake_account, amount_sol, signature, created_at)
                 VALUES (?, ?, ?, ?, ?, ?)"
            )
            .bind(&stake_id)
            .bind(&payload.from_pubkey)
            .bind(&stake_account)
            .bind(payload.amount_sol)
            .bind(&stake_sig)
            .bind(&Utc::now().to_rfc3339())
            .execute(&state.db)
            .await;
        }
        Err(e) => {
            println!("Staking failed (deposit still recorded): {}", e);
        }
    }

    (
        StatusCode::OK,
        Json(DepositResponse {
            success: true,
            message: format!("Deposit of {} SOL confirmed and staked", payload.amount_sol),
        }),
    )
}