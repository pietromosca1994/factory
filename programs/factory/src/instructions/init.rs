use anchor_lang::prelude::*;

use crate::program_accounts::{TokenRegistry, Whitelist};

pub fn init(_ctx: Context<Init>) -> Result<()> {
    // to be checked if initialized 

    // whitelist initialization
    let whitelist = &mut _ctx.accounts.whitelist;
    whitelist.authorized_users = Vec::new();
    whitelist.authorized_users.push(*_ctx.accounts.signer.key);

    msg!("authority account:    {}", _ctx.accounts.authority.key());
    msg!("whitelist account:    {}", _ctx.accounts.whitelist.key());

    Ok(())
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    ///CHECK: pda
    #[account(
        init_if_needed,     // created if needed //TODO remove if in production
        payer = payer,      // User funds the account
        space = 8 + 32,     // Account size
        seeds = [b"authority"], // PDA Seeds signer.key().as_ref()
        bump,
        rent_exempt = enforce 
    )]
    pub authority:  UncheckedAccount<'info>, // Account<'info, UpdateAuthorityAccount>,
    // #[account(
    //     init_if_needed,
    //     payer = payer,
    //     space = 8 + TokenRegistry::INIT_SPACE,
    //     seeds = [b"token_registry"],
    //     bump
    // )]
    // pub token_registry: Account<'info, TokenRegistry>,
    #[account(
        init_if_needed,
        seeds = [b"whitelist"],
        bump,
        payer = payer,
        space = 8 + 4 + 32 * 5, //TODO Adjust space depending on max users
        rent_exempt = enforce 
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}