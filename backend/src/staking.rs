use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    stake::{
        self,
        state::Authorized,
        state::Lockup,
    },
    transaction::Transaction,
    system_instruction,
};
use std::str::FromStr;

// top validator on devnet for testing
//const VALIDATOR_VOTE_ACCOUNT: &str = "3ZT31jkAGhUaw8jsy4bTknwBMP8i4Eueh52By4zXcsVw";
const VALIDATOR_VOTE_ACCOUNT: &str = "2f9C9AU8nFRKUub8NHToNiZzcwmYiNeipVuP8akKgRVv";

pub async fn stake_native(
    rpc_url: &str,
    protocol_keypair: &Keypair,
    amount_sol: f64,
) -> Result<(String, String)> {
    let client = RpcClient::new(rpc_url.to_string());
    let lamports = (amount_sol * 1_000_000_000.0) as u64;

    // generate a new stake account keypair
    let stake_account = Keypair::new();

    println!("Creating stake account: {}", stake_account.pubkey());

    let authorized = Authorized {
        staker: protocol_keypair.pubkey(),
        withdrawer: protocol_keypair.pubkey(),
    };

    let lockup = Lockup::default();

    // get minimum rent for stake account
    let rent = client.get_minimum_balance_for_rent_exemption(
        std::mem::size_of::<stake::state::StakeStateV2>()
    )?;

    let total_lamports = lamports + rent;

    let recent_blockhash = client.get_latest_blockhash()?;

    // build transaction with 3 instructions:
    // 1. create stake account
    // 2. initialize it
    // 3. delegate to validator
    let vote_pubkey = Pubkey::from_str(VALIDATOR_VOTE_ACCOUNT)?;

    let create_ix = system_instruction::create_account(
        &protocol_keypair.pubkey(),
        &stake_account.pubkey(),
        total_lamports,
        std::mem::size_of::<stake::state::StakeStateV2>() as u64,
        &stake::program::id(),
    );

    let init_ix = stake::instruction::initialize(
        &stake_account.pubkey(),
        &authorized,
        &lockup,
    );

    let delegate_ix = stake::instruction::delegate_stake(
        &stake_account.pubkey(),
        &protocol_keypair.pubkey(),
        &vote_pubkey,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[create_ix, init_ix, delegate_ix],
        Some(&protocol_keypair.pubkey()),
        &[protocol_keypair, &stake_account],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;

    println!("Staked {} SOL natively. Signature: {}", amount_sol, signature);
    println!("Stake account: {}", stake_account.pubkey());

    
    Ok((signature.to_string(), stake_account.pubkey().to_string()))
}