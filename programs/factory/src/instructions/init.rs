use anchor_lang::prelude::*;

use crate::program_accounts::{TokenRegistry, UpdateAuthorityAccount};

pub fn init(_ctx: Context<Init>) -> Result<()> {
    // to be checked if initialized 
    Ok(())
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,               // Create account
        payer = payer,  // User funds the account
        space = 8 + 32,     // Account size
        seeds = [b"update_authority",  signer.key().as_ref()], // PDA Seeds
        bump
    )]
    pub update_authority_pda: Account<'info, UpdateAuthorityAccount>,
    pub system_program: Program<'info, System>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + TokenRegistry::INIT_SPACE,
        seeds = [b"token_registry"],
        bump
    )]
    pub token_registry: Account<'info, TokenRegistry>,
}