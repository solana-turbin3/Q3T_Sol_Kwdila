use std::thread::current;

use anchor_lang::prelude::*;

use crate::{constants::MIN_PLAYERS, state::{game_data, GameData, PlayerData}, GameErrorCode, GameState};

#[derive(Accounts)]
pub struct StartGame<'info> {
    #[account(mut)]
    pub host: Signer<'info>,
    #[account(
        mut,
        seeds=[
            b"secret_hitler",
            host.key().to_bytes().as_ref()
        ],
        bump=game_data.bump,
        constraint = game_data.game_state == GameState::Setup @GameErrorCode::InvalidGameState,
        constraint = game_data.active_player_count >= MIN_PLAYERS, //minimum players to play
    )]
    pub game_data: Account<'info, GameData>,
    #[account(
        seeds = [            
            game_data.key().to_bytes().as_ref(),
            host.key().to_bytes().as_ref()
        ],
        bump = player_data.bump,
        constraint = !player_data.is_eliminated @GameErrorCode::EiminatedPlayer //TODO there must be no inactive host 
    )]
    pub player_data: Account<'info, PlayerData>,
}

impl<'info> StartGame<'info> {
    pub fn start(&mut self) ->Result<()>{
        let game = &mut self.game_data;
        let current_time = Clock::get()?.unix_timestamp;

        game.game_state = GameState::ChancellorNomination;
        game.turn_started_at = Some(current_time);

        Ok(())
    }
}