use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, InitSpace)]
pub struct TokenInfo {
    #[max_len(50)]
    pub id: String,
    pub token_mint: Pubkey, // Token mint address
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct TokenMeta {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub description: String,
    // pub attributes: Vec<Attribute>,
    pub properties: Vec<Property>
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct Property {
    pub key: String,
    pub value: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct TransferTokenParams {
    pub id: String,
}
