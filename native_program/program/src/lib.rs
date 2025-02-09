// cargo build-sbf
// solana program deploy ./target/deploy/program.so
// solana address -k ./target/deploy/program-keypair.json

#![allow(unexpected_cfgs)]
use solana_program::{
    account_info::AccountInfo, 
    entrypoint::ProgramResult, 
    entrypoint, msg, 
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey, 
    _accounts: &[AccountInfo], 
    _instruction_data: &[u8]
) -> ProgramResult {
    msg!("Hello, World!");
    Ok(())
}
