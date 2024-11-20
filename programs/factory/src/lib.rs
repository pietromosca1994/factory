use anchor_lang::prelude::*;

pub mod program_accounts;
pub mod instructions;
pub mod program_types;
pub mod program_events;

use instructions::*;

use program_types::{InitTokenParams, TransferTokenParams, TokenInfo};
use program_events::NFTCreationEvent;
use program_accounts::TokenRegistry;

declare_id!("GGVMhhhYAUuioAw1npbv1NMFBuXs8icgGYQVfjoyEoup");

#[program]
mod factory {
    use super::*;

    pub fn init(_ctx: Context<Init>) -> Result<()> {
        // let token_registry = &ctx.accounts.token_registry;
        // msg!("TokenRegistry owner: {}", token_registry.owner);
        Ok(())
    }

    pub fn mint_nft(_ctx: Context<MintNFT>, init_token_params: InitTokenParams) -> Result<()> {
        mint_nft::mint_nft(_ctx, init_token_params)
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

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(
        init_if_needed,
        payer = admin,
        space = 8 + TokenRegistry::INIT_SPACE,
        seeds = [b"token_registry"],
        bump
    )]
    pub token_registry: Account<'info, TokenRegistry>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
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
