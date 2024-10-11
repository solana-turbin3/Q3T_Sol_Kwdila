use anchor_lang::prelude::*;

use crate::error::EscrowError;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    
    pub mint_b: Pubkey,
    pub recieve_amount: u64,
    pub expiry: u64,
    pub is_mutable: bool,
    pub escrow_bump: u8,
    pub vault_bump: u8,
}
impl Escrow {
    // 8 bytes descriminator, 32 bytes Pubkey, 8 bytes u64, 1 byte u8,! byte bool;
    pub const LEN: usize = 8 + 3 * 32 + 3 * 8 + 3 * 1 + 1;

    pub fn check_expiry(&self) -> Result<()> {
        require!(self.expiry > Clock::get()?.slot, EscrowError::Expired);
        Ok(())
    }

    pub fn set_expiry_relative(&mut self, expiry: u64) -> Result<()> {
        require!(expiry.lt(&100_000), EscrowError::MaxExpiryExceeded);
        self.set_expiry_absolute(match expiry > 0 {
            true => Clock::get()?.slot + expiry,
            false => 0,
        });
        Ok(())
    }

    pub fn set_expiry_absolute(&mut self, expiry: u64) {
        self.expiry = expiry;
    }
}
