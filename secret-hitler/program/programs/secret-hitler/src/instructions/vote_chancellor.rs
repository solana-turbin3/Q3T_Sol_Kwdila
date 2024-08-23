use anchor_lang::prelude::*;

use crate::state::{GameData, Nomination, PlayerData};
use crate::{GameErrorCode, GameState, PlayerVote};

#[derive(Accounts)]
pub struct LeaveGame<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        seeds = [
            game_data.key().to_bytes().as_ref(),
            player.key().to_bytes().as_ref()
        ],
        bump = player_data.bump,
        constraint = !player_data.is_eliminated @GameErrorCode::EiminatedPlayer,
    )]
    pub player_data: Account<'info, PlayerData>,
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
        constraint = game_data.active_players.contains(player.key) @GameErrorCode::PlayerNotInGame,
    )]
    pub game_data: Account<'info, GameData>,
}

impl<'info> LeaveGame<'info> {
    pub fn vote(&mut self, vote: PlayerVote) -> Result<()> {
        let nomination = &mut self.nomination;
        let game = &mut self.game_data;
        let num_players = game.active_players.len() as u8;

        let total_votes = nomination.ja + nomination.nein;
        require!(total_votes <= num_players, GameErrorCode::MaxVotesReached);

        // get player index from all active players in game
        let player_index = game
            .active_players
            .iter()
            .position(|key| key == self.player.key)
            .ok_or(GameErrorCode::PlayerNotInGame)? as u64;

        // make sure the player has not voted already
        require!(
            !nomination.voters_index.contains(&player_index),
            GameErrorCode::PlayerAlreadyVoted
        );

        match vote {
            PlayerVote::Nein => nomination.nein += 1,
            PlayerVote::Ja => nomination.ja += 1,
        }

        if nomination.nein > num_players.div_ceil(2) - 1 {
            game.failed_elections += 1;
            game.game_state = GameState::ChancellorNomination
        }

        if nomination.ja > num_players.div_ceil(2) - 1 {
            game.previous_chancellor_index = game.current_chancellor_index;
            game.current_chancellor_index = Some(nomination.nominee_index);
            game.game_state = GameState::LegislativePresident;
        }

        Ok(())
    }
}
