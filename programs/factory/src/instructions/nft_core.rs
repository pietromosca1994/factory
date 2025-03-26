use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{spl_token_2022},
};
use mpl_core::{
    types::{PluginAuthorityPair, Plugin, Attributes, PluginAuthority, Attribute}, 
    instructions::{UpdatePluginV1CpiBuilder, CreateV2CpiBuilder},
};

pub use crate::program_events::{NFTCreationEvent, NFTUpdateEvent};
pub use crate::program_types::{TokenMeta, TokenInfo};
pub use crate::program_accounts::{UpdateAuthorityAccount, Whitelist};
pub use crate::program_utils::{get_core_asset_meta, check_if_in_whitelist};
pub use crate::program_errors::ProgramError;

pub fn mint_nft_core(_ctx: Context<MintNFTCore>, token_meta: TokenMeta) -> Result<()> {
    msg!("asset account:    {}", _ctx.accounts.asset.key().to_string());
    msg!("update_authority: {}", _ctx.accounts.update_authority.key());

    let whitelist = &_ctx.accounts.whitelist;
    check_if_in_whitelist(*_ctx.accounts.signer.key, whitelist)?;

    // the name of the token is "token_<token_id>"
    // let name: String = format!("{}{}", String::from("token_"), token_meta.name);
    
    let signers: &[&[&[u8]]] = &[
        &[
            b"asset",
            token_meta.name.as_bytes(),
            &[_ctx.bumps.asset],
        ],
        // update authority is not necessarily needed for signing this
        &[
            b"update_authority",
            &[_ctx.bumps.update_authority],
        ],
    ];

    let mut attribute_list: Vec<Attribute> = Vec::new();
    for element in token_meta.properties.iter(){
        attribute_list.push(
            Attribute {
                key: element.key.clone(),
                value: element.value.clone(),
            }
        );
    }

    let asset_plugins = vec![
        PluginAuthorityPair {
            plugin: Plugin::Attributes(Attributes { attribute_list }),
            authority: Some(PluginAuthority::UpdateAuthority),
        },
    ];

    // Interact with mpl_core to create the NFT
    // Ref: https://developers.metaplex.com/core/create-asset
    let _=CreateV2CpiBuilder::new(&_ctx.accounts.mpl_core_program.to_account_info())
        .asset(&_ctx.accounts.asset.to_account_info())
        .payer(&_ctx.accounts.signer.to_account_info())
        .name(token_meta.name.clone())
        .uri(token_meta.uri.clone())
        .system_program(&_ctx.accounts.system_program.to_account_info())
        .owner(Some(&_ctx.accounts.update_authority.to_account_info()))
        .plugins(asset_plugins)
        // .update_authority(Some(&_ctx.accounts.signer.to_account_info()))
        .update_authority(Some(&_ctx.accounts.update_authority.to_account_info()))
        // .invoke();
        .invoke_signed(signers)?;

    // Emit NFT creation event
    emit!(NFTCreationEvent {
        name: token_meta.name.clone(),
        asset: _ctx.accounts.asset.key(),
        signer: _ctx.accounts.signer.key(),
    });

    Ok(())
}

pub fn update_properties_nft_core(_ctx: Context<UpdatePropertiesNFTCore>, token_meta: TokenMeta) -> Result<()> {
    msg!("asset account:    {}", _ctx.accounts.asset.key().to_string());
    msg!("update_authority: {}", _ctx.accounts.update_authority.key());

    let whitelist = &_ctx.accounts.whitelist;
    check_if_in_whitelist(*_ctx.accounts.signer.key, whitelist)?;
    
    // the name of the token is "token_<token_id>"
    // let name: String = format!("{}{}", String::from("token_"), token_meta.name);
    msg!("asset account: {}", _ctx.accounts.asset.key().to_string());
    
    let signers: &[&[&[u8]]] = &[
        &[
            b"asset",
            token_meta.name.as_bytes(),
            &[_ctx.bumps.asset],
        ],
        &[
            b"update_authority",
            &[_ctx.bumps.update_authority],
        ],
    ];

    let mut attribute_list: Vec<Attribute> = Vec::new();
    for element in token_meta.properties.iter(){
        attribute_list.push(
            Attribute {
                key: element.key.clone(),
                value: element.value.clone(),
            }
        );
    }

    // Interact with mpl_core to update the properties of the NFT
    // Ref: https://developers.metaplex.com/core/create-asset
    let _=UpdatePluginV1CpiBuilder::new(&_ctx.accounts.mpl_core_program.to_account_info())
        .asset(&_ctx.accounts.asset.to_account_info())
        .payer(&_ctx.accounts.signer.to_account_info())
        .system_program(&_ctx.accounts.system_program.to_account_info())
        .plugin(Plugin::Attributes(Attributes { attribute_list: attribute_list.clone() }))
        .authority(Some(&_ctx.accounts.update_authority.to_account_info()))
        // .invoke();
        .invoke_signed(signers)?;

    // Emit NFT update event
    emit!(NFTUpdateEvent {
        name: token_meta.name.clone(),
        asset: _ctx.accounts.asset.key(),
        signer: _ctx.accounts.signer.key(),
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    token_meta: TokenMeta
)]
pub struct MintNFTCore<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    ///CHECK: check pda address
    #[account(
        mut,
        seeds = [b"asset", token_meta.name.as_bytes()],
        bump,                         
    )]
    pub asset: UncheckedAccount<'info>,
    ///CHECK: check pda address 
    #[account(
        mut,
        seeds = [b"update_authority"],
        bump,                         
    )]
    pub update_authority:  UncheckedAccount<'info>,
    ///CHECK: check pda address 
    #[account(
        mut,
        seeds = [b"whitelist"],
        bump,                         
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    
    /// CHECK: address
    #[account(mut, address = spl_token_2022::ID)]
    pub token_program: UncheckedAccount<'info>,
    // pub token_program: Interface<'info, TokenInterface>,

    /// CHECK: address
    #[account(mut, address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
#[instruction(
    token_meta: TokenMeta
)]
pub struct UpdatePropertiesNFTCore<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    ///CHECK: check
    #[account(
        mut,
        seeds = [b"asset", token_meta.name.as_bytes()],
        bump,                         
    )]
    pub asset: UncheckedAccount<'info>,
    ///CHECK: check pda address 
    #[account(
        mut,
        seeds = [b"update_authority"],
        bump,                         
    )]
    pub update_authority:  UncheckedAccount<'info>, // pub update_authority: Account<'info, UpdateAuthorityAccount>,
    ///CHECK: check pda address 
    #[account(
        mut,
        seeds = [b"whitelist"],
        bump,                         
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    
    /// CHECK: address
    #[account(mut, address = spl_token_2022::ID)]
    pub token_program: UncheckedAccount<'info>,
    // pub token_program: Interface<'info, TokenInterface>,

    /// CHECK: address
    #[account(mut, address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}