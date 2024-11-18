use anchor_lang::prelude::*;

use crate::program_types::TokenInfo;

// https://www.anchor-lang.com/docs/space
#[account]
#[derive(InitSpace)]
pub struct TokenRegistry {
    #[max_len(10)]                  //TODO set the length of this vector
    pub tokens: Vec<TokenInfo>,
    pub total_tokens: u64,
}