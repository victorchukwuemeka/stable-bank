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
    pub wallet_balance_sol: f64,
    pub total_deposited : f64,
    pub total_withdrawn : f64,
    pub active_staked_sol: Vec<StakeInfo>,
    pub total_staked_sol: f64,
    pub estimated_yield_sol: f64,

}


#[derive(Serialize)]
pub struct StakeInfo{
    pub stake_account : String,
    pub amount_sol : f64,
    pub staked_at : String,
}

pub async fn handle_balance(
    Path(pubkey): Path<String>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<BalanceResponse>) {
    let client = RpcClient::new(state.rpc_url.clone());
    
     //todo 
     // get the wallet balance 
    let wallet_balance = match Pubkey::from_str(&pubkey){
        Ok(p) => client.get_balance(&p).unwrap_or(0) as f64 / 1_000_000_000.0,
        Err(_) => 0.0,
    };

    //todo get the user record fromt the db
    let users_record = sqlx::query_as::<_,(f64,f64)>(
        "SELECT total_deposited, total_withdrawn FROM users WHERE pubkey = ?"
    )
    .bind(&pubkey)
    .fetch_optional(&state.db)
    .await
    .unwrap_or(None);
    
    let (total_deposited, total_withdrawn) = users_record.unwrap_or((0.0, 0.0));
     
     let stakes = sqlx::query_as::<_, (String, f64, String)>(
        "SELECT stake_account, amount_sol, created_at FROM stake_accounts WHERE pubkey = ?"
    )
    .bind(&pubkey)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();



    //todo 
    let mut total_staked_sol = 0.0;
    let mut stake_infos = vec![];


    for(stake_account, amount_sol, created_at) in &stakes{
        let live_balance = match Pubkey::from_str(stake_account){
            Ok(p) => client.get_balance(&p).unwrap_or(0) as f64 / 1_000_000_000.0,
            Err(_) => *amount_sol,
        };

        total_staked_sol += live_balance;
        
        stake_infos.push(StakeInfo {
            stake_account: stake_account.clone(),
            amount_sol: live_balance,
            staked_at: created_at.clone(),
        });

    };


   

    let estimated_yield_sol = (total_staked_sol - total_deposited).max(0.0);
    
    (
        StatusCode::OK,
        Json(BalanceResponse {
            pubkey: pubkey.to_string(),
            wallet_balance_sol: wallet_balance,
            total_deposited,
            total_withdrawn,
            active_staked_sol: stake_infos,
            total_staked_sol,
            estimated_yield_sol,
        }),
    )
    
}