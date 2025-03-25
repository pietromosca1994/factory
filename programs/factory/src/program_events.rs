use anchor_lang::prelude::*;

#[event]
pub struct NFTCreationEvent {
    pub name: String,
    pub asset: Pubkey,
    pub signer: Pubkey
}

#[event]
pub struct NFTUpdateEvent {
    pub name: String,
    pub asset: Pubkey,
    pub signer: Pubkey
}