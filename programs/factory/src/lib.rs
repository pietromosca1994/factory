use anchor_lang::prelude::*;
use anchor_spl::
    token_interface::{TokenInterface, Mint};
use mpl_token_metadata::{
    accounts::{Metadata, MasterEdition},
    instructions::CreateV1CpiBuilder,
    types::{PrintSupply, TokenStandard, Creator},
};
use mpl_token_metadata::ID as MPL_TOKEN_METADATA_ID;

pub mod program_accounts;
pub mod instructions;
pub mod program_types;
pub mod program_events;

use program_types::{InitTokenParams, TokenInfo};
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

    pub fn mint_nft(_ctx: Context<MintNFT>, token_params: InitTokenParams) -> Result<()> {

        // set metadata
        let name: String = format!("{}{}", String::from("token_"), token_params.id);
        let symbol: String = String::from("VLT");
    
        // get accounts
        let mint = &_ctx.accounts.mint;
        let signer = &_ctx.accounts.signer;
        let token_program = &_ctx.accounts.token_program;
        let token_metadata_program = &_ctx.accounts.token_metadata_program;
        let system_program = &_ctx.accounts.system_program;
        let metadata=&_ctx.accounts.metadata;
        let master_edition=&_ctx.accounts.master_edition;
        let token_registry=& mut _ctx.accounts.token_registry;

        // https://docs.rs/mpl-core/latest/mpl_core/instructions/struct.CreateV1CpiBuilder.html
        // https://developers.metaplex.com/guides/rust/how-to-cpi-into-a-metaplex-program
        // mpl-core ref: https://docs.rs/mpl-core/0.7.0/mpl_core/instructions/index.html
        
        // Derive the bump for master_edition and metadata PDAs
        let (master_edition_id, master_edition_bump) = MasterEdition::find_pda(&mint.key());
        let (metadata_id, metadata_bump) = Metadata::find_pda(&mint.key());
        
        msg!("token id:                         {}", token_params.id);
        msg!("mint account:                     {}", mint.key().to_string());
        msg!("master edition account:           {}", master_edition_id.key().to_string());
        msg!("metadata account:                 {}", metadata_id.key().to_string());
        // msg!("master_edition_bump:      {}", master_edition_bump.to_string());
        // msg!("metadata_bump:            {}", metadata_bump.to_string());
        
        // Construct the signers array with the proper seed and bump for each PDA
        let signers: &[&[&[u8]]] = &[
            // Seed and bump for the mint PDA
            &["mint".as_bytes(), token_params.id.as_bytes(), &[_ctx.bumps.mint]],
        ];
    
        // set creatots (optional)
        let creators = vec![Creator{
            address: signer.key(),
            verified: false,
            share: 100,
        }];
    
        // let uses=Uses{
        //     use_method: 
        //     remaining:
        //     total:
        // };
    
        // mint token
        let _ = CreateV1CpiBuilder::new(&token_metadata_program.to_account_info())
                                    .payer(&signer.to_account_info())
                                    .mint(&mint.to_account_info(), true)
                                    .update_authority(&mint.to_account_info(), true)
                                    .authority(&signer.to_account_info())
                                    .name(name)
                                    .symbol(symbol)
                                    .uri(token_params.uri)
                                    .seller_fee_basis_points(0)
                                    .metadata(&metadata.to_account_info())
                                    .system_program(&system_program.to_account_info())
                                    .sysvar_instructions(&system_program.to_account_info())
                                    .print_supply(PrintSupply::Limited(1)) 
                                    .master_edition(Some(&master_edition.to_account_info())) //     https://developers.metaplex.com/token-metadata/print
                                    .token_standard(TokenStandard::NonFungible)
                                    .spl_token_program(Some(&token_program.to_account_info()))
                                    .creators(creators)
                                    .is_mutable(true)
                                    // .uses(uses)
                                    .invoke_signed(signers)?;
    
        // Emit the NFTCreationEvent
        emit!(NFTCreationEvent {
            id: token_params.id.clone(),
            mint: _ctx.accounts.mint.key(),
            owner: _ctx.accounts.signer.key(),
            
        });

        // Update token registry
        let token_info = TokenInfo{
            id: token_params.id.clone(),
            token_mint: mint.key()
        };
        token_registry.tokens.push(token_info);
        token_registry.total_tokens+=1;
        msg!("Total tokens: {}", token_registry.total_tokens);

        Ok(())
    }
    
}

#[derive(Accounts)]
#[instruction(
    token_params: InitTokenParams
)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init, // the program creates this account
        payer = signer,
        mint::decimals = 0,
        mint::authority = signer.key(),
        mint::freeze_authority = signer.key(),
        seeds = [b"mint", token_params.id.as_bytes()],    // the PDA is seeded with the token ID to ensure uniqueness
        bump,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    
    /// CHECK: address
    #[account(
        mut,
        address = Metadata::find_pda(&mint.key()).0,
    )]
    pub metadata: AccountInfo<'info>,

    /// CHECK: address
    #[account(
        mut,
        address = MasterEdition::find_pda(&mint.key()).0,
    )]
    pub master_edition: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    
    /// CHECK: address
    #[account(
    mut,
        address = MPL_TOKEN_METADATA_ID,
    )]
    pub token_metadata_program: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"token_registry"],
        bump
    )]
    pub token_registry: Account<'info, TokenRegistry>,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(
        init_if_needed,
        payer = admin,
        space = 8 + TokenRegistry::INIT_SPACE, // Assume max 100 tokens
        seeds = [b"token_registry"],
        bump
    )]
    pub token_registry: Account<'info, TokenRegistry>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}