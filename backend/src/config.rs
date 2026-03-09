use std::env;

pub struct Config {
    pub rpc_url: String,
    pub wallet_path: String,
}

impl Config {
    pub fn load() -> Self {
        dotenv::dotenv().ok();
        Self {
            rpc_url: env::var("RPC_URL")
                .unwrap_or("https://api.devnet.solana.com".to_string()),
            wallet_path: env::var("WALLET_PATH")
                .unwrap_or("./keypair.json".to_string()),
        }
    }
}