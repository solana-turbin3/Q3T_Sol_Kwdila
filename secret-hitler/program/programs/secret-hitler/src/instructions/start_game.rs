use anchor_lang::prelude::*;

use crate::{state::{GameData, PlayerData}, GameErrorCode, GameState};

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
        constraint = game_data.game_state == GameState::Setup @GameErrorCode::InvalidGameState
    )]
    pub game_data: Account<'info, GameData>,
    #[account(

        seeds = [            
            game_data.key().to_bytes().as_ref(),
            host.key().to_bytes().as_ref()
        ],
        bump = player_data.bump,
        constraint = player_data.is_active @GameErrorCode::InactivePlayer
    )]
    pub player_data: Account<'info, PlayerData>,
}

impl<'info> StartGame<'info> {
    pub fn 
}