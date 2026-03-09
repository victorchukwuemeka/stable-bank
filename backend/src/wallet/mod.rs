use anyhow::Result;
use solana_sdk::signature::{Keypair, Signer, read_keypair_file};
use solana_client::rpc_client::RpcClient;
use std::path::Path;
use std::fs;

pub fn load_or_create_wallet(path: &str) -> Result<Keypair> {
    if Path::new(path).exists() {
        println!("🔑 Loading existing wallet from {}", path);
        let keypair = read_keypair_file(path)
            .map_err(|e| anyhow::anyhow!("Failed to read keypair: {}", e))?;
        Ok(keypair)
    } else {
        println!("🆕 No wallet found — generating new wallet...");
        let keypair = Keypair::new();
        let bytes = keypair.to_bytes();
        let json = serde_json::to_string(&bytes.to_vec())?;
        fs::write(path, json)?;
        println!("✅ Wallet saved to {}", path);
        Ok(keypair)
    }
}

pub fn get_balance(rpc_url: &str, keypair: &Keypair) -> Result<f64> {
    let client = RpcClient::new(rpc_url.to_string());
    let pubkey = keypair.pubkey();
    let lamports = client.get_balance(&pubkey)?;
    let sol = lamports as f64 / 1_000_000_000.0;
    Ok(sol)
}