use anchor_lang::prelude::*;

use crate::{
    state::{GameData, Nomination},
    GameErrorCode, GameState,
    GameState::*,
};

#[derive(Accounts)]
pub struct EliminatePlayer<'info> {
    // no need to check if they are in the game
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
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
        constraint = ![FascistVictoryElection,FascistVictoryPolicy,LiberalVictoryExecution,LiberalVictoryPolicy,Setup]
            .contains(&game_data.game_state) @GameErrorCode::InvalidGameState,
    )]
    pub game_data: Account<'info, GameData>,
}
impl<'info> EliminatePlayer<'info> {
    pub fn eliminate_player(&mut self) -> Result<()> {
        let game = &mut self.game_data;

        let current_time = Clock::get()?.unix_timestamp;
        let turn_start_time = game
            .turn_started_at
            .ok_or(GameErrorCode::TurnStartTimeNotFound)?;

        require!(
            current_time - turn_start_time > 0,
            GameErrorCode::TurnNotFinished
        );

        let mut indices_to_remove: Vec<u64> = Vec::new();
        let mut inactive_goverment = false;

        match game.game_state {
            GameState::ChancellorVoting => {
                let voters = &self.nomination.voters_index;
                for (index, key) in game.active_players.iter().enumerate() {
                    let index_u64 = index as u64;

                    if !voters.contains(&index_u64) {
                        indices_to_remove.push(index_u64);
                        if game.is_chancellor(key) | game.is_president(key) {
                            inactive_goverment = true;
                        }
                    }
                }
            }
            // in all other allowed GameSates either the chancellor or president are supposed to be active
            _ => {
                inactive_goverment = true;
            }
        }

        // Remove players in reverse order to avoid shifting errors
        for index in indices_to_remove.iter().rev() {
            self.game_data.active_players.remove(*index as usize);
        }

        if inactive_goverment {
            self.game_data.next_president();
            self.game_data.next_turn(GameState::ChancellorNomination)?;
        };

        Ok(())
    }
}
