pub use crate::errors::GameErrorCode;
pub use anchor_lang::prelude::*;

pub mod constants;
pub mod enums;
pub mod errors;
pub mod helpers;
pub mod instructions;
pub mod state;

pub use enums::*;
pub use instructions::*;

declare_id!("AnvTCoxxQzscMBqVPtdEsc6it1U39rmqt6rQvfCt9Uac");

#[program]
pub mod secret_hitler {
    use super::*;

    pub fn initialize_game(
        ctx: Context<InitializeGame>,
        max_players: u8,
        turn_duration: i64,
        entry_deposit: Option<u64>,
        bet_amount: Option<u64>,
    ) -> Result<()> {
        ctx.accounts.init_game(
            max_players,
            turn_duration,
            entry_deposit,
            bet_amount,
            ctx.bumps,
        )?;
        Ok(())
    }
    pub fn join_game(ctx: Context<JoinGame>) -> Result<()> {
        ctx.accounts.add_player(ctx.bumps)?;
        Ok(())
    }
    pub fn leave_game(ctx: Context<LeaveGame>) -> Result<()> {
        ctx.accounts.remove_player()?;
        Ok(())
    }
    pub fn start_game(ctx: Context<StartGame>) -> Result<()> {
        ctx.accounts.start()?;
        Ok(())
    }
}
