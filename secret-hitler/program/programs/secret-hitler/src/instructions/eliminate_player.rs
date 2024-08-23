use anchor_lang::prelude::*;

use crate::{
    state::{GameData, Nomination},
    GameErrorCode,
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

        let mut indices_to_remove: Vec<usize> = Vec::new();
        let current_government_indices = [
            Some(game.current_president_index),
            game.current_chancellor_index,
        ];
        let mut inactive_goverment = false;

        for (index, _) in game.active_players.iter().enumerate() {
            if current_government_indices.contains(&Some(index)) {
                inactive_goverment = true;
            }
            if !voters.contains(&index) {
                indices_to_remove.push(index);
            }
        }

        // Remove players in reverse order using the indices from indices_to_remove
        for index in indices_to_remove.iter().rev() {
            self.game_data.active_players.remove(*index);
        }

        if inactive_goverment {
            self.update_goverment()
        };

        Ok(())
    }

    pub fn update_goverment(&mut self) {
        self.game_data.next_president();
        self.game_data.current_chancellor_index = None;
    }
}
