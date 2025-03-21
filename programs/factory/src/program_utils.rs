use mpl_token_metadata::accounts::Metadata;
use solana_program::account_info::AccountInfo;

pub fn get_core_asset_meta(metadata_account: &AccountInfo) -> Metadata {
    let data = metadata_account.try_borrow_data().unwrap(); // Borrow account data, unwrap the result
    
    // Deserializing the metadata from the account data
    Metadata::safe_deserialize(&mut data.as_ref()).unwrap() // Unwrap deserialization result
}