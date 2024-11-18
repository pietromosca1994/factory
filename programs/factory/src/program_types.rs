use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, InitSpace)]
pub struct TokenInfo {
    #[max_len(50)]
    pub id: String,
    pub token_mint: Pubkey,         // Token mint address
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitTokenParams {
    pub id: String,
    pub uri: String,
}