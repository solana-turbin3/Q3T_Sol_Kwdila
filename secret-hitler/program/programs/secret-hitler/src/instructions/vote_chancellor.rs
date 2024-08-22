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
        constraint = !player_data.is_eliminated @GameErrorCode::EiminatedPlayer
    )]
    pub player_data: Account<'info, PlayerData>,
    #[account(
        mut,
        seeds =[
            b"chancellor_nomination",
            game_data.key().to_bytes().as_ref()
            ],
        bump = nomination.bump
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
        constraint = game_data.players.contains(player.key) @GameErrorCode::PlayerNotInGame,
    )]
    pub game_data: Account<'info, GameData>,
}

impl<'info> LeaveGame<'info> {
    pub fn vote(&mut self, vote: PlayerVote) -> Result<()> {
        let nomination = &mut self.nomination;
        let game = &mut self.game_data;

        match vote {
            PlayerVote::Nein => nomination.nein += 1,
            PlayerVote::Ja => nomination.ja += 1,
        }

        let total_votes = nomination.ja + nomination.nein;
        require!(
            total_votes <= game.active_player_count,
            GameErrorCode::MaxVotesReached
        );

        if nomination.nein > game.active_player_count.div_ceil(2) - 1 {
            game.failed_elections += 1;
            game.game_state = GameState::ChancellorNomination
        }

        if nomination.ja > game.active_player_count.div_ceil(2) - 1 {
            todo!() //TODO handle passing vote
        }
        Ok(())
    }
}
