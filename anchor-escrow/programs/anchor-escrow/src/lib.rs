pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("3ZSratuRHNTmgE9YHA6HanPGkBU1wfDT1ZgwqfsyC1yy");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(
        ctx: Context<Make>,
        seed: u64,
        recieve_amount: u64,
        duration: u64,
        is_mutable: bool,
    ) -> Result<()> {
        ctx.accounts
            .initialize_escrow(seed, recieve_amount, duration, is_mutable, ctx.bumps)?;
        ctx.accounts.deposit_into_escrow()?;

        Ok(())
    }
    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.check_expiry()?;
        ctx.accounts.deposit_into_vault()?;
        ctx.accounts.withdraw_from_vault()?;
        ctx.accounts.close_vault()?;
        Ok(())
    }
    pub fn update(
        ctx: Context<Update>,
        offer_amount: u64,
        expiry: u64,
        is_mutable: bool,
    ) -> Result<()> {
        ctx.accounts.update(offer_amount, expiry, is_mutable)?;
        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        ctx.accounts.withdraw_from_vault()?;
        ctx.accounts.close_vault()?;
        Ok(())
    }
}
