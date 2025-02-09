#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("2oodd2y46ysZUPTwGb2KSzqak7S6pBgg25s6WEXnk56F");

#[program]
pub mod anchor_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
