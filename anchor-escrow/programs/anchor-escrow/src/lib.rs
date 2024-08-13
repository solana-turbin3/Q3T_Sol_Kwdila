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

    pub fn make(ctx: Context<Make>, seed: u64, recieve_amount: u64) -> Result<()> {
        ctx.accounts
            .initialize_escrow(seed, ctx.bumps, recieve_amount)?;

        ctx.accounts.deposit_into_escrow()?;

        Ok(())
    }
    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit_into_vault()?;
        ctx.accounts.withdraw_and_close_vault()?;
        Ok(())
    }
}
