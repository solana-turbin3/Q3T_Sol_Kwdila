use anchor_lang::error_code;

#[error_code]
pub enum EscrowError {
    #[msg("Unable to get vault bump")]
    VaultBumpError,
    #[msg("Unable to get escrow bump")]
    EscrowBumpError,
    #[msg("Your expiration is too far into the future")]
    MaxExpiryExceeded,
    #[msg("Escrow has expired")]
    Expired,
    #[msg("Escrow.is_mut must be true to update escrow")]
    EscrowNotMutable,
    #[msg("Mint Provided does not match the mint in escrow")]
    MintMismatch,
}
