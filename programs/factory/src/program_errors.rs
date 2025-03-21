use anchor_lang::error_code;

#[error_code]
pub enum ProgramError {
    #[msg("Insufficient funds to complete the transaction.")]
    InsufficientFunds,

    #[msg("Invalid account data.")]
    InvalidAccountData,

    #[msg("Unauthorized access.")]
    Unauthorized,
}