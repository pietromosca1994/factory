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

    pub fn mint_nft(_ctx: Context<MintNFT>, token_meta: TokenMeta) -> Result<()> {
        mint_nft::mint_nft(_ctx, token_meta)
    }

    pub fn mint_nft_core(_ctx: Context<MintNFTCore>, token_meta: TokenMeta) -> Result<()> {
        mint_nft_core::mint_nft_core(_ctx, token_meta)
    }

    pub fn update_properties_nft_core(_ctx: Context<UpdatePropertiesNFTCore>, token_meta: TokenMeta) -> Result<()> {
        update_properties_nft_core::update_properties_nft_core(_ctx, token_meta)
    }
    // pub fn transfer_nft(_ctx: Context<BurnNFT>, transfer_token_params: BurnTokenParams) -> Result<()> {

    //     // get accounts
    //     let signer = &_ctx.accounts.signer;
    //     let token_program = &_ctx.accounts.token_program;
    //     let token_metadata_program = &_ctx.accounts.token_metadata_program;
    //     let system_program = &_ctx.accounts.system_program;
    //     let metadata=&_ctx.accounts.metadata;
    //     let master_edition=&_ctx.accounts.master_edition;
    //     let token_registry=& mut _ctx.accounts.token_registry;
    //     let program_id = _ctx.program_id;
    //     let new_owner=&_ctx.accounts.new_owner;

    //     let (mint, mint_bump) = Pubkey::find_program_address(&["mint".as_bytes(), burn_token_params.id.as_bytes()], program_id);
        
    //     // Construct the signers array with the proper seed and bump for each PDA
    //     let signers: &[&[&[u8]]] = &[
    //         // Seed and bump for the mint PDA
    //         &["mint".as_bytes(), burn_token_params.id.as_bytes(), &[mint_bump]],
    //     ];

    //     let _ = TransferV1CpiBuilder::new(&token_metadata_program.to_account_info())
    //     .mint(mint)
    //     .payer(&signer.to_account_info())
    //     .authority(&signer.to_account_info())
    //     .destination_owner(&destination_owner.to_account_info())
    //     .system_program(&system_program.to_account_info())
    //     .spl_token_program(Some(&token_program.to_account_info()))
    //     .invoke_signed(signers)?;

    //     Ok(())
    // }

}

// #[derive(Accounts)]
// #[instruction(
//     transfer_token_params: TransferTokenParams
// )]
// pub struct TransferNFT<'info> {
//     #[account(mut)]
//     pub signer: Signer<'info>,

//     /// CHECK: address
//     #[account()]
//     pub new_owner: UncheckedAccount<'info>,

//     pub rent: Sysvar<'info, Rent>,
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, Token2022>,
    
//     /// CHECK: address
//     #[account(
//     mut,
//         address = MPL_TOKEN_METADATA_ID,
//     )]
//     pub token_metadata_program: UncheckedAccount<'info>,

//     #[account(
//         mut,
//         seeds = [b"token_registry"],
//         bump
//     )]
//     pub token_registry: Account<'info, TokenRegistry>,
// }
