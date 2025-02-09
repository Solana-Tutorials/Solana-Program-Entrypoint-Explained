#![allow(unexpected_cfgs)]
use pinocchio::{
  account_info::AccountInfo,
  entrypoint,
  msg,
  ProgramResult,
  pubkey::Pubkey
};

entrypoint!(call_this_what_you_want);
      
pub fn call_this_what_you_want(
  _first: &Pubkey,
  _second: &[AccountInfo],
  _third: &[u8],
) -> ProgramResult {
  msg!("Hello from my program!");
  Ok(())
}