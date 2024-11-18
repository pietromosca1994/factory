use anchor_lang::prelude::*;

#[account]
pub struct TokenRegistry {
    // A list of token info
    pub tokens: Vec<TokenInfo>,
    // The admin or owner of the registry
    pub admin: Pubkey,
    // Total number of tokens tracked
    pub total_tokens: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct TokenInfo {
    pub token_mint: Pubkey,         // Token mint address
    pub token_vault: Pubkey,        // Token vault address
    pub pool_creator: Pubkey,       // Creator of the pool
    pub lp_mint: Pubkey,            // Liquidity provider mint
    pub auth_bump: u8,              // Authority bump
    pub observation_key: Pubkey,    // Observation key
    pub lp_supply: u64,             // Liquidity pool supply
    pub status: u8,                 // Status of the token (e.g., 0 for active, 1 for inactive)
    pub protocol_fees_token0: u64,  // Protocol fees for token0
    pub protocol_fees_token1: u64,  // Protocol fees for token1
    pub fund_fees_token0: u64,      // Fund fees for token0
    pub fund_fees_token1: u64,      // Fund fees for token1
    pub open_time: i64,             // Open timestamp
}