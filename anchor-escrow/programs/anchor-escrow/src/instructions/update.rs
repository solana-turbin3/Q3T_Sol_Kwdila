use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::{error::EscrowError, state::Escrow};

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub new_taker_token: Account<'info, Mint>,
    #[account(
        mut,
        has_one = maker,
        seeds = [b"escrow", maker.key.as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.escrow_bump,
        // escrow is_mutable field has to be true in order to update it
        constraint = escrow.is_mutable @EscrowError::EscrowNotMutable,
    )]
    pub escrow: Box<Account<'info, Escrow>>,
}

impl<'info> Update<'info> {
    pub fn update(&mut self, recieve_amount: u64, expiry: u64, is_mutable: bool) -> Result<()> {
        let escrow = &mut self.escrow;
        escrow.mint_b = *self.new_taker_token.to_account_info().key;
        escrow.recieve_amount = recieve_amount;
        escrow.set_expiry_relative(expiry)?;
        escrow.is_mutable = is_mutable;
        Ok(())
    }
}
