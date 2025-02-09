use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::LAMPORTS_PER_SOL,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::{str::FromStr, thread, time::Duration};

const PROGRAM_ID: &str = "9gZP6t9z4x23wXp7MWzcpbjoUwSCfz2Cp9pBojRufu3p";
const RPC_URL: &str = "http://127.0.0.1:8899";

fn main() -> Result<()> {
    // Create connection
    let commitment_config = CommitmentConfig::confirmed();
    let connection = RpcClient::new_with_commitment(RPC_URL.to_string(), commitment_config);

    // Setup payer and request airdrop
    let payer = Keypair::new();
    let airdrop_signature = connection.request_airdrop(&payer.pubkey(), LAMPORTS_PER_SOL)?;
    connection.confirm_transaction(&airdrop_signature)?;
    thread::sleep(Duration::from_secs(1));

    // Create instruction
    let program_id = Pubkey::from_str(PROGRAM_ID)?;
    let instruction = Instruction::new_with_bytes(program_id, &[], vec![]);
    
    // Create and send transaction
    let recent_blockhash = connection.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    
    let signature = connection.send_and_confirm_transaction(&transaction)?;
    println!(
        "Transaction: https://explorer.solana.com/tx/{}?cluster=custom",
        signature
    );
    Ok(())
}