use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{MintTo, mint_to},
    token_interface::{TokenInterface, Mint, TokenAccount},
    associated_token::{AssociatedToken}};
use mpl_token_metadata::{
    accounts::{Metadata, MasterEdition},
    instructions::{CreateV1CpiBuilder},
    types::{PrintSupply, TokenStandard, Creator},
};

pub use crate::program_events::NFTCreationEvent;
pub use crate::program_types::{TokenMeta, TokenInfo};
pub use crate::program_accounts::TokenRegistry;

pub fn mint_nft(_ctx: Context<MintNFT>, token_meta: TokenMeta) -> Result<()> {

    // set metadata
    let name: String = format!("{}{}", String::from("token_"), token_meta.name);
    let symbol: String = String::from("VLT");

    // msg!("token id:                         {}", token_meta.name);
    // msg!("mint account:                     {}", _ctx.accounts.mint.key().to_string());
    // msg!("token account:                    {}", _ctx.accounts.token_account.key().to_string());
    // msg!("master edition account:           {}", _ctx.accounts.master_edition.key().to_string());
    // msg!("metadata account:                 {}", _ctx.accounts.metadata.key().to_string());
    // msg!("token program:                    {}", _ctx.accounts.token_program.key().to_string());
    
    // Construct the signers array with the proper seed and bump for each PDA
    let signers: &[&[&[u8]]] = &[
        // Seed and bump for the mint PDA
        &["mint".as_bytes(), token_meta.name.as_bytes(), &[_ctx.bumps.mint]],
    ];

    // let _ = MintV1CpiBuilder::new(&_ctx.accounts.token_metadata_program.to_account_info())
    //                             .token(&_ctx.accounts.token_account.to_account_info())
    //                             .token_owner(Some(&_ctx.accounts.token_program.to_account_info()))
    //                             .metadata(&_ctx.accounts.metadata.to_account_info())
    //                             .master_edition(Some(&_ctx.accounts.master_edition.to_account_info()))
    //                             .mint(&_ctx.accounts.mint.to_account_info())
    //                             .authority(&_ctx.accounts.signer.to_account_info())
    //                             .payer(&_ctx.accounts.signer.to_account_info())
    //                             .system_program(&_ctx.accounts.system_program.to_account_info())
    //                             .spl_token_program(&_ctx.accounts.token_program.to_account_info())
    //                             .spl_ata_program(&_ctx.accounts.associated_token_program.to_account_info())
    //                             .sysvar_instructions(&_ctx.accounts.system_program.to_account_info())
    //                             .amount(1)
    //                             .invoke_signed(signers);

    // Mint 1 token to the associated token account
    let cpi_ctx = CpiContext::new(
        _ctx.accounts.token_program.to_account_info(), 
        MintTo {
            mint: _ctx.accounts.mint.to_account_info(),
            to: _ctx.accounts.token_account.to_account_info(),
            authority: _ctx.accounts.signer.to_account_info(),
        });
    mint_to(cpi_ctx, 1)?; // Mint exactly 1 token (NFTs have 0 decimals)
    
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

    // mint token with mpl-metadata     
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
                                .uri(token_meta.uri)
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
                                .print_supply(PrintSupply::Zero)
                                .decimals(0)
                                // .uses(uses)
                                .invoke_signed(signers)?;
    
    // mint token with mpl-core
    // let _ = CreateV1CpiBuilder::new(&_ctx.accounts.token_metadata_program.to_account_info())
    //                             .name(name)
    //                             .payer(&_ctx.accounts.signer.to_account_info())
    //                             .asset(&_ctx.accounts.mint.to_account_info())
    //                             .update_authority(Some(&_ctx.accounts.mint.to_account_info()))
    //                             .authority(Some(&_ctx.accounts.signer.to_account_info()))
    //                             .uri(token_meta.uri)
    //                             .system_program(&_ctx.accounts.system_program.to_account_info())
    //                             // .uses(uses)
    //                             .invoke_signed(signers)?;

    // Emit the NFTCreationEvent
    emit!(NFTCreationEvent {
        name: token_meta.name.clone(),
        asset: _ctx.accounts.mint.key(),
        signer: _ctx.accounts.signer.key(),
        
    });

    // Update token registry
    let token_info = TokenInfo{
        id: token_meta.name.clone(),
        token_mint: _ctx.accounts.mint.key()
    };
    _ctx.accounts.token_registry.tokens.push(token_info);
    _ctx.accounts.token_registry.total_tokens+=1;
    msg!("Total tokens: {}", _ctx.accounts.token_registry.total_tokens);

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    token_meta: TokenMeta
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
        seeds = [b"mint", token_meta.name.as_bytes()],    // the PDA is seeded with the token ID to ensure uniqueness
        bump,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
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
    // https://solana.com/developers/courses/token-extensions/token-extensions-onchain
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    
    /// CHECK: address
    #[account(
        mut,
        address = mpl_token_metadata::ID,
    )]
    pub token_metadata_program: UncheckedAccount<'info>,

    /// CHECK: address
    #[account(
        address = solana_program::sysvar::instructions::ID
    )]
    pub sysvar_instructions: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"token_registry"],
        bump
    )]
    pub token_registry: Account<'info, TokenRegistry>,
}