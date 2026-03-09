mod config;
mod wallet;
mod routes;

use config::Config;
use wallet::{load_or_create_wallet, get_balance};
use solana_sdk::signature::Signer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load();

    println!("🏦 StableBank Backend Starting...");
    println!("🌐 Connecting to: {}", config.rpc_url);

    let keypair = load_or_create_wallet(&config.wallet_path)?;

    println!("📬 Protocol Wallet: {}", keypair.pubkey());

    match get_balance(&config.rpc_url, &keypair) {
        Ok(balance) => println!("💰 Balance: {} SOL", balance),
        Err(e) => println!("⚠️  Could not fetch balance: {}", e),
    }

    println!("✅ StableBank is ready.");
    Ok(())
}