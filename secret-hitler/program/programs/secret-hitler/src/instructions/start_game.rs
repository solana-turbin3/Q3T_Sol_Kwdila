use anchor_lang::prelude::*;

use crate::{constants::MIN_PLAYERS, state::{ GameData, PlayerData}, GameErrorCode, GameState};

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
        constraint = game_data.active_players.len() >= MIN_PLAYERS as usize, //minimum players to play
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

        game.game_state = GameState::ChancellorNomination;
        game.reset_turn_timer()?;

        Ok(())
    }
}