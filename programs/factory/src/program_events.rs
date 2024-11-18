use anchor_lang::prelude::*;

#[event]
pub struct NFTCreationEvent {
    pub id: String,
    pub mint: Pubkey,
    pub owner: Pubkey
}