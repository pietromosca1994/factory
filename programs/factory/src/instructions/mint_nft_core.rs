use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{spl_token_2022},
};
use mpl_core::{
    types::{PluginAuthorityPair, Plugin, Attributes, PluginAuthority, Attribute}, 
    instructions::{CreateV2CpiBuilder}
};

pub use crate::program_events::NFTCreationEvent;
pub use crate::program_types::{TokenMeta, TokenInfo};
pub use crate::program_accounts::TokenRegistry;

pub fn mint_nft_core(ctx: Context<MintNFTCore>, token_meta: TokenMeta) -> Result<()> {
    // the name of the token is "token_<token_id>"
    // let name: String = format!("{}{}", String::from("token_"), token_meta.name);
    msg!("asset account: {}", ctx.accounts.asset.key().to_string());
    
    let signers: &[&[&[u8]]] = &[
        &[
            b"asset",
            token_meta.name.as_bytes(),
            &[ctx.bumps.asset],
        ],
        // update authority is not necessarily needed for signing this
        &[
            b"update_authority",
            ctx.accounts.signer.key.as_ref(),
            &[ctx.bumps.update_authority_pda],
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

    // attribute_list.push(Attribute {
    //     key: String::from("SOH [%]"),
    //     value: String::from("100"),
    // });
    // attribute_list.push(Attribute {
    //     key: String::from("Cycles [EFC]"),
    //     value: String::from("1000"),
    // });

    let asset_plugins = vec![
        PluginAuthorityPair {
            plugin: Plugin::Attributes(Attributes { attribute_list }),
            authority: Some(PluginAuthority::UpdateAuthority),
        },
    ];

    // Interact with mpl_core to create the NFT
    // Ref: https://developers.metaplex.com/core/create-asset
    let _=CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
        .asset(&&ctx.accounts.asset.to_account_info())
        .payer(&ctx.accounts.signer.to_account_info())
        .name(token_meta.name.clone())
        .uri(token_meta.uri.clone())
        .system_program(&ctx.accounts.system_program.to_account_info())
        .owner(Some(&ctx.accounts.signer.to_account_info()))
        .plugins(asset_plugins)
        .update_authority(Some(&ctx.accounts.signer.to_account_info()))
        // .update_authority(Some(&ctx.accounts.update_authority_pda.to_account_info()))
        // .invoke();
        .invoke_signed(signers)?;

    // Emit NFT creation event
    emit!(NFTCreationEvent {
        name: token_meta.name.clone(),
        asset: ctx.accounts.asset.key(),
        owner: ctx.accounts.signer.key(),
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
        seeds = [b"update_authority", signer.key().as_ref()],
        bump,                         
    )]
    pub update_authority_pda: UncheckedAccount<'info>,
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