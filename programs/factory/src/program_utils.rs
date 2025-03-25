use anchor_lang::prelude::*;
use mpl_token_metadata::accounts::Metadata;
use solana_program::account_info::AccountInfo;

use crate::program_accounts::Whitelist;
use crate::program_errors::ProgramError;

pub fn get_core_asset_meta(metadata_account: &AccountInfo) -> Metadata {
    let data = metadata_account.try_borrow_data().unwrap(); // Borrow account data, unwrap the result
    
    // Deserializing the metadata from the account data
    Metadata::safe_deserialize(&mut data.as_ref()).unwrap() // Unwrap deserialization result
}

pub fn derive_whitelist_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    let seeds: &[&[u8]] = &[b"whitelist"];
    let (pda, bump) = Pubkey::find_program_address(seeds, program_id);
    (pda, bump)
}

pub fn derive_asset_pda(program_id: &Pubkey, name: String) -> (Pubkey, u8) {
    let seeds: &[&[u8]] = &[b"asset", name.as_bytes()];
    let (pda, bump) = Pubkey::find_program_address(seeds, program_id);
    (pda, bump)
}

pub fn check_if_in_whitelist(user: Pubkey, whitelist_account: &Whitelist) -> Result<()> {
    if !whitelist_account.authorized_users.contains(&user) {
        return Err(error!(ProgramError::UserNotWhitelisted));
    }
    Ok(())
}