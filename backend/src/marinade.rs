use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;
use solana_sdk::pubkey::Pubkey;



const MARINADE_API: &str = "https://quote-api.marinade.finance/v1";

#[derive(Serialize)]
struct StakeRequest {
    pub amount: u64,
    pub wallet_address: String,
}

#[derive(Deserialize, Debug)]
pub struct StakeResponse {
    pub transaction: String,
    pub msol_amount: Option<u64>,
}

pub async fn stake_sol(
    rpc_url: &str,
    keypair: &Keypair,
    amount_sol: f64,
) -> Result<String> {
    let client = Client::new();
    let lamports = (amount_sol * 1_000_000_000.0) as u64;

    println!("Requesting stake transaction from Marinade for {} SOL", amount_sol);

    let response = client
        .post(format!("{}/stake", MARINADE_API))
        .json(&StakeRequest {
            amount: lamports,
            wallet_address: keypair.pubkey().to_string(),
        })
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("Marinade API error: {}", error_text));
    }

    let stake_response: StakeResponse = response.json().await?;

    println!("Got transaction from Marinade, signing and submitting...");

    // decode the transaction
    let tx_bytes = base64::decode(&stake_response.transaction)?;
    let mut transaction: Transaction = bincode::deserialize(&tx_bytes)?;

    // sign with protocol wallet
    let rpc_client = RpcClient::new(rpc_url.to_string());
    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    transaction.sign(&[keypair], recent_blockhash);

    // submit
    let signature = rpc_client.send_and_confirm_transaction(&transaction)?;

    println!("Staked {} SOL on Marinade. Signature: {}", amount_sol, signature);

    Ok(signature.to_string())
}


