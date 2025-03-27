use anchor_lang::prelude::*;
use mpl_core::{
    types::Attribute,
};

#[event]
pub struct NFTCreationEvent {
    pub name: String,
    pub asset: Pubkey,
    pub owner: Pubkey,
    pub update_authority: Pubkey
}

#[event]
pub struct NFTUpdateEvent {
    pub name: String,
    pub asset: Pubkey,
    pub owner: Pubkey,
    pub update_authority: Pubkey,
    pub uri: String,
}

#[event]
pub struct NFTPropertiesUpdateEvent {
    pub name: String,
    pub asset: Pubkey,
    pub owner: Pubkey,
    pub update_authority: Pubkey,
    pub properties: Vec<Attribute>
}

#[event]
pub struct NFTTransferEvent {
    pub name: String,
    pub asset: Pubkey,
    pub owner: Pubkey,
    pub update_authority: Pubkey
}

#[event]
pub struct NFTBurnEvent {
    pub name: String,
    pub asset: Pubkey,
    pub owner: Pubkey,
    pub update_authority: Pubkey
}

#[event]
pub struct AddedUserToWhitelist {
    pub user: Pubkey
}

#[event]
pub struct RemovedUserFromWhitelist {
    pub user: Pubkey
}



