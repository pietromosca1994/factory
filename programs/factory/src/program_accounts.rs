use anchor_lang::prelude::*;

use crate::program_types::TokenInfo;

// https://www.anchor-lang.com/docs/space
#[account]
#[derive(InitSpace)]
pub struct TokenRegistry {
    #[max_len(100)]                  //TODO set the length of this vector
    pub tokens: Vec<TokenInfo>,
    pub total_tokens: u64,
}

// Define a custom PDA struct
#[account]
pub struct UpdateAuthorityAccount {
    pub owner: Pubkey, // Account owner
}

#[account]
pub struct Whitelist {
    pub authorized_users: Vec<Pubkey>, // List of allowed users
}