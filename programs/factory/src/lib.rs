#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

pub mod program_accounts;
pub mod instructions;
pub mod program_types;
pub mod program_events;
pub mod program_errors;
pub mod program_utils;

use instructions::*;

use program_types::TokenMeta;

declare_id!("374sEAYBSbxqHpzn9rsgkd2ECW1HgtiNFThC8h8txjgp");

#[program]
mod factory {
    use super::*;

    pub fn init(_ctx: Context<Init>) -> Result<()> {
        // Assuming init::init returns Result<(), Error>
        init::init(_ctx)?;  // The `?` operator will return the error if there is one
        
        // If you need to log or process something further, you can do that here
        // let token_registry = &ctx.accounts.token_registry;
        // msg!("TokenRegistry owner: {}", token_registry.owner);
    
        Ok(())  // Return Ok if everything went well
    }

    // pub fn mint_nft(_ctx: Context<MintNFT>, token_meta: TokenMeta) -> Result<()> {
    //     nft_core::mint_nft(_ctx, token_meta)
    // }

    pub fn mint_nft_core(_ctx: Context<MintNFTCore>, token_meta: TokenMeta) -> Result<()> {
        nft_core::mint_nft_core(_ctx, token_meta)
    }

    pub fn update_nft_core(_ctx: Context<UpdateNFTCore>, token_meta: TokenMeta) -> Result<()> {
        nft_core::update_nft_core(_ctx, token_meta)
    }

    pub fn transfer_nft_core(_ctx: Context<TransferNFTCore>, token_meta: TokenMeta) -> Result<()> {
        nft_core::transfer_nft_core(_ctx, token_meta)
    }

    pub fn burn_nft_core(_ctx: Context<BurnNFTCore>, token_meta: TokenMeta) -> Result<()> {
        nft_core::burn_nft_core(_ctx, token_meta)
    }

    pub fn update_properties_nft_core(_ctx: Context<UpdatePropertiesNFTCore>, token_meta: TokenMeta) -> Result<()> {
        nft_core::update_properties_nft_core(_ctx, token_meta)
    }

    pub fn add_to_whitelist(_ctx: Context<AddToWhitelist>, user: Pubkey) -> Result<()> {
        whitelist::add_to_whitelist(_ctx, user)
    }

    pub fn remove_from_whitelist(_ctx: Context<RemoveFromWhitelist>, user: Pubkey) -> Result<()> {
        whitelist::remove_from_whitelist(_ctx, user)
    }

}