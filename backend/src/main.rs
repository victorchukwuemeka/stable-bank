mod config;
mod wallet;
mod routes;
mod db;
mod marinade;
mod staking;



use config::Config;
use wallet::{load_or_create_wallet, get_balance};
use solana_sdk::signature::Signer;
use axum::{Router, routing::get, routing::post};
use std::sync::Arc;
use sqlx::SqlitePool;
use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;

pub struct AppState {
    pub rpc_url: String,
    pub wallet_path: String,
    pub db: SqlitePool,
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load();

    

    println!("StableBank Backend Starting...");
    println!("Connecting to: {}", config.rpc_url);


    let keypair = load_or_create_wallet(&config.wallet_path)?;

    println!("📬 Protocol Wallet: {}", keypair.pubkey());

    match get_balance(&config.rpc_url, &keypair) {
        Ok(balance) => println!(" Balance: {} SOL", balance),
        Err(e) => println!(" Could not fetch balance: {}", e),
    }

    let db = match db::init_db().await{
        Ok(pool)=>{
            println!("Database initialized successfully");
            pool
        }
        Err(e)=>{
            println!("Database error: {}", e);
            return Err(e);
        }
    };


    let state = Arc::new(AppState{
        rpc_url: config.rpc_url,
        wallet_path: config.wallet_path,
        db,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/deposit", post(routes::deposit::handle_deposit))
        .route("/balance/:pubkey", get(routes::balance::handle_balance))
        .route("/withdraw", post(routes::withdraw::handle_withdraw))
        .route("/withdraw/claim", post(routes::withdraw::handle_claim))
        .route("/config", get(routes::config::handle_config))
        .layer(cors)
        .with_state(state);

    println!("Server running on http://localhost:3002");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

