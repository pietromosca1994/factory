use anchor_lang::prelude::*;

#[event]
pub struct NFTCreationEvent {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub id: String,
}