use anchor_lang::prelude::*;

mod contexts;
use contexts::*;
mod state;

declare_id!("GBXjiMa13stmnTPQfe23rQDwuVCitr1zA5vgeCXozhYf");

#[program]
pub mod vault {
    use super::*;
    pub fn initialize(ctx: Context<InitializeVault>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }
    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }
    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }
    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()
    }
}
