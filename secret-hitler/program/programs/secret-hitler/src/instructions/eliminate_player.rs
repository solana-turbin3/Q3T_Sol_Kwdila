use std::ops::Index;

use anchor_lang::prelude::*;

use crate::{
    state::{GameData, Nomination},
    GameErrorCode, GameState,
};

#[derive(Accounts)]
pub struct EliminatePlayer<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        mut,
        seeds =[
            b"chancellor_nomination",
            game_data.key().to_bytes().as_ref()
        ],
        bump = nomination.bump,
    )]
    pub nomination: Account<'info, Nomination>,
    #[account(
        mut,
        seeds = [
            b"secret_hitler",
            game_data.host.to_bytes().as_ref(),
        ],
        bump = game_data.bump,
        constraint = game_data.game_state == GameState::ChancellorVoting @GameErrorCode::InvalidGameState,
    )]
    pub game_data: Account<'info, GameData>,
}
impl<'info> EliminatePlayer<'info> {
    pub fn eliminate_player(&mut self) -> Result<()> {
        let game = &mut self.game_data;
        let voters = &self.nomination.voters_index;

        let current_time = Clock::get()?.unix_timestamp;

        let turn_start_time = game.turn_started_at.ok_or(GameErrorCode::TurnNotFinished)?;

        require!(
            current_time - turn_start_time > 0,
            GameErrorCode::TurnNotFinished
        );

        for (index, player) in game.players.iter().enumerate() {
            if !voters.contains(&index) {}
        }

        Ok(())
    }
}
