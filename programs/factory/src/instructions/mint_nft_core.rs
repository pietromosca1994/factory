use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{MintTo, mint_to, Token2022},
    
    token_interface::{TokenInterface, Mint, TokenAccount}
};
use mpl_core::{
    types::{PluginAuthorityPair, Plugin, Attributes, PluginAuthority, Attribute},
    accounts::{BaseAssetV1, BaseCollectionV1}, 
    instructions::CreateV1CpiBuilder
};
// use mpl_token_metadata::{
//     accounts::{Metadata, MasterEdition},
//     instructions::{TransferV1CpiBuilder, MintV1CpiBuilder},
//     types::{PrintSupply, TokenStandard, Creator},
// };

pub use crate::program_events::NFTCreationEvent;
pub use crate::program_types::{InitTokenParams, TokenInfo};
pub use crate::program_accounts::TokenRegistry;

pub fn mint_nft_core(_ctx: Context<MintNFTCore>, init_token_params: InitTokenParams) -> Result<()>{

    // set metadata
    let name: String = format!("{}{}", String::from("token_"), init_token_params.id);

    msg!("asset account:                     {}", _ctx.accounts.asset.key().to_string());
    let signers: &[&[&[u8]]] = &[
        // Seed and bump for the mint PDA
        &["asset".as_bytes(), init_token_params.id.as_bytes(), &[_ctx.bumps.asset]],
    ];

    let mut attribute_list: Vec<Attribute> = Vec::new();
    attribute_list.push(Attribute {
        key: String::from("attribute1"),
        value: String::from("1")
    });
    attribute_list.push(Attribute {
        key: String::from("attribute2"),
        value: String::from("2")
    });

    let mut asset_plugins = Vec::new();
    asset_plugins.push(
        PluginAuthorityPair {
            plugin: Plugin::Attributes(
                Attributes {attribute_list: attribute_list}
            ),
            authority: Some(PluginAuthority::UpdateAuthority)
        }
    );

    // https://developers.metaplex.com/core/getting-started/rust
    let _ = CreateV1CpiBuilder::new(&_ctx.accounts.mpl_core_program.to_account_info())
                                .asset(&_ctx.accounts.asset.to_account_info())
                                .payer(&_ctx.accounts.signer.to_account_info())
                                .name(name)
                                .uri(init_token_params.uri)
                                // .authority(Some(&_ctx.accounts.signer.to_account_info()))
                                // .update_authority(Some(&_ctx.accounts.signer.to_account_info()))
                                .system_program(&_ctx.accounts.system_program.to_account_info())
                                .owner(Some(&_ctx.accounts.signer.to_account_info()))
                                .plugins(asset_plugins)
                                // .invoke()?;
                                .invoke_signed(signers)?;
    
    // Emit the NFTCreationEvent
    emit!(NFTCreationEvent {
        id: init_token_params.id.clone(),
        mint: _ctx.accounts.asset.key(),
        owner: _ctx.accounts.signer.key(),
        
    });
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    init_token_params: InitTokenParams
)]
pub struct MintNFTCore<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,

    // #[account(  
    //     // mut,
    //     init_if_needed,
    //     payer = payer,
    //     owner = mpl_core::ID,
    //     // update_authority = mpl_core::ID,
    //     seeds = [b"asset", init_token_params.id.as_bytes()],    // the PDA is seeded with the token ID to ensure uniqueness
    //     bump,
    //     space = 160
    // )]
    // pub asset: Account<'info, BaseAssetV1>,
    /// CHECK: address
    #[account(
        mut,
        seeds = [b"asset", init_token_params.id.as_bytes()],    // the PDA is seeded with the token ID to ensure uniqueness
        bump,
    )]
    // pub asset: Account<'info, BaseAssetV1>,
    pub asset: UncheckedAccount<'info>,

    /// CHECK: address
    // #[account(mut)]
    // pub mint: Signer<'info>,
    // #[account(
    //     mut,
    //     constraint = collection.update_authority=signer.key()
    // )]
    // pub collection: account<'info, BaseCollectionV1>;

    // #[account(
    //     init_if_needed,
    //     payer = signer,
    //     associated_token::mint = mint,
    //     associated_token::authority = signer,
    //     associated_token::token_program = token_program
    // )]
    // pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    // https://solana.com/developers/courses/token-extensions/token-extensions-onchain
    pub token_program: Interface<'info, TokenInterface>,
    
    /// CHECK: address
    #[account(
        mut,
        address = mpl_core::ID,
    )]
    pub mpl_core_program: UncheckedAccount<'info>,

    // #[account(
    //     mut,
    //     seeds = [b"token_registry"],
    //     bump
    // )]
    // pub token_registry: Account<'info, TokenRegistry>,
}