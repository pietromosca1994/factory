use anchor_lang::prelude::*;

use crate::program_accounts::Whitelist;
use crate::program_utils::check_if_in_whitelist;

pub fn add_to_whitelist(_ctx: Context<AddToWhitelist>, user: Pubkey) -> Result<()> {
    let whitelist = &mut _ctx.accounts.whitelist;

    check_if_in_whitelist(*_ctx.accounts.signer.key, whitelist)?;
    
    // Check if the user is already in the whitelist
    if !whitelist.authorized_users.contains(&user) {
        // Add user to whitelist
        whitelist.authorized_users.push(user);
    }

    Ok(())
}

pub fn remove_from_whitelist(_ctx: Context<RemoveFromWhitelist>, user: Pubkey) -> Result<()> {
    let whitelist = &mut _ctx.accounts.whitelist;

    check_if_in_whitelist(*_ctx.accounts.signer.key, whitelist)?;
    
    // Remove the user from the whitelist
    whitelist.authorized_users.retain(|&u| u != user);

    Ok(())
}

#[derive(Accounts)]
pub struct AddToWhitelist<'info> {
    #[account(signer)]
    pub signer: Signer<'info>,
    ///CHECK: check pda address 
    #[account(
        mut,
        seeds = [b"whitelist"],
        bump,                         
    )]
    pub whitelist: Account<'info, Whitelist>,
}

#[derive(Accounts)]
pub struct RemoveFromWhitelist<'info> {
    #[account(signer)]
    pub signer: Signer<'info>,
    ///CHECK: check pda address 
    #[account(
        mut,
        seeds = [b"whitelist"],
        bump,                         
    )]
    pub whitelist: Account<'info, Whitelist>,
}