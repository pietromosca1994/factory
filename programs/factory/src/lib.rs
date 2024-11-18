use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{MintTo, mint_to},
    token_interface::{TokenInterface, Mint, TokenAccount},
    associated_token::{AssociatedToken}};
use mpl_token_metadata::{
    accounts::{Metadata, MasterEdition},
    instructions::{CreateV1CpiBuilder, TransferV1CpiBuilder},
    types::{PrintSupply, TokenStandard, Creator},
};
use mpl_token_metadata::ID as MPL_TOKEN_METADATA_ID;

pub mod program_accounts;
pub mod instructions;
pub mod program_types;
pub mod program_events;

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

        // set metadata
        let name: String = format!("{}{}", String::from("token_"), init_token_params.id);
        let symbol: String = String::from("VLT");

        msg!("token id:                         {}", init_token_params.id);
        msg!("mint account:                     {}", _ctx.accounts.mint.key().to_string());
        msg!("token account:                    {}", _ctx.accounts.token_account.key().to_string());
        msg!("master edition account:           {}", _ctx.accounts.master_edition.key().to_string());
        msg!("metadata account:                 {}", _ctx.accounts.metadata.key().to_string());
        
        // Mint 1 token to the associated token account
        let cpi_ctx = CpiContext::new(
            _ctx.accounts.token_program.to_account_info(), 
            MintTo {
                mint: _ctx.accounts.mint.to_account_info(),
                to: _ctx.accounts.token_account.to_account_info(),
                authority: _ctx.accounts.signer.to_account_info(),
            });
        mint_to(cpi_ctx, 1)?; // Mint exactly 1 token (NFTs have 0 decimals)
        
        // Construct the signers array with the proper seed and bump for each PDA
        let signers: &[&[&[u8]]] = &[
            // Seed and bump for the mint PDA
            &["mint".as_bytes(), init_token_params.id.as_bytes(), &[_ctx.bumps.mint]],
        ];

        // set creatots (optional)
        let creators = vec![Creator{
            address: _ctx.program_id.key(),
            verified: false,
            share: 100,
        }];
    
        // let uses=Uses{
        //     use_method: 
        //     remaining:
        //     total:
        // };
    
        // mint token        
        // https://docs.rs/mpl-core/latest/mpl_core/instructions/struct.CreateV1CpiBuilder.html
        // https://developers.metaplex.com/guides/rust/how-to-cpi-into-a-metaplex-program
        // mpl-core ref: https://docs.rs/mpl-core/0.7.0/mpl_core/instructions/index.html
        let _ = CreateV1CpiBuilder::new(&_ctx.accounts.token_metadata_program.to_account_info())
                                    .payer(&_ctx.accounts.signer.to_account_info())
                                    .mint(&_ctx.accounts.mint.to_account_info(), true)
                                    .update_authority(&_ctx.accounts.mint.to_account_info(), true)
                                    .authority(&_ctx.accounts.signer.to_account_info())
                                    .name(name)
                                    .symbol(symbol)
                                    .uri(init_token_params.uri)
                                    .seller_fee_basis_points(0)
                                    .metadata(&_ctx.accounts.metadata.to_account_info())
                                    .system_program(&_ctx.accounts.system_program.to_account_info())
                                    .sysvar_instructions(&_ctx.accounts.system_program.to_account_info())
                                    .print_supply(PrintSupply::Zero )
                                    .master_edition(Some(&_ctx.accounts.master_edition.to_account_info())) //     https://developers.metaplex.com/token-metadata/print
                                    .token_standard(TokenStandard::NonFungible)
                                    .spl_token_program(Some(&_ctx.accounts.token_program.to_account_info()))
                                    .creators(creators)
                                    .is_mutable(true)
                                    // .uses(uses)
                                    .invoke_signed(signers)?;
    
        // Emit the NFTCreationEvent
        emit!(NFTCreationEvent {
            id: init_token_params.id.clone(),
            mint: _ctx.accounts.mint.key(),
            owner: _ctx.accounts.signer.key(),
            
        });

        // Update token registry
        let token_info = TokenInfo{
            id: init_token_params.id.clone(),
            token_mint: _ctx.accounts.mint.key()
        };
        _ctx.accounts.token_registry.tokens.push(token_info);
        _ctx.accounts.token_registry.total_tokens+=1;
        msg!("Total tokens: {}", _ctx.accounts.token_registry.total_tokens);

        Ok(())
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

#[derive(Accounts)]
#[instruction(
    init_token_params: InitTokenParams
)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = signer.key(),
        mint::freeze_authority = signer.key(),
        mint::token_program = token_program,
        seeds = [b"mint", init_token_params.id.as_bytes()],    // the PDA is seeded with the token ID to ensure uniqueness
        bump,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

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
    pub associated_token_program: Program<'info, AssociatedToken>,
    
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
#[instruction(
    transfer_token_params: TransferTokenParams
)]
pub struct TransferNFT<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: address
    #[account()]
    pub new_owner: UncheckedAccount<'info>,

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
