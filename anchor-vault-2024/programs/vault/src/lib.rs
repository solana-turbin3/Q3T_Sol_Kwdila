use anchor_lang::prelude::*;

mod contexts;
use contexts::*;
mod state;

declare_id!("Gh5ShrdXL8zFGH5UyfAc12WjhRpKv3Kcvz4hQ1jKf74C");

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
