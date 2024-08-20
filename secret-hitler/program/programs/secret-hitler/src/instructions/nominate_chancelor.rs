use anchor_lang::prelude::*;

use crate::enums::GameState;
use crate::state::{game_data::GameData, player_data::PlayerData};
use crate::{nominate_chancelor, GameErrorCode};

#[derive(Accounts)]
pub struct NominateChancellor<'info> {
    #[account(
        mut,
    )]
    pub president: Signer<'info>,
    #[account(
        seeds = [            
            game_data.key().to_bytes().as_ref(),
            president.key().to_bytes().as_ref(),
        ],
        bump = player_data.bump,
        constraint = player_data.is_in_game @GameErrorCode::PlayerNotInGame
    )]
    pub player_data: Account<'info, PlayerData>,
    #[account(
        seeds = [
            b"secret_hitler",
            game_data.host.to_bytes().as_ref(),
        ],
        bump = game_data.bump,

        constraint = president.key().eq(
            game_data.players.get(game_data.current_president_index).unwrap() // this is checked
        ) @GameErrorCode::PlayerNotInGame,

        constraint = 
            [GameState::Setup,GameState::PostLegislative]
                .contains(&game_data.game_state) @GameErrorCode::InvalidGameState
    )]
    pub game_data: Account<'info, GameData>,
}

impl<'info> NominateChancellor<'info> {
    pub fn nominate_chancellor(
        &mut self,
        nominated_chancellor_index: usize,
    ) -> Result<()> {
        let game = &mut self.game_data;
        require!(nominated_chancellor_index < 10 ,GameErrorCode::PlayerNotInGame);
        let nominate_chancelor = game.players.get(nominated_chancellor_index).unwrap();

        let nomination_check: bool = 
            match game.previous_chancellor_index.is_some(){
                true => {
                    match game.player_count <= 5 {
                        true => {
                            game.players.get(game.previous_chancellor_index.unwrap()).unwrap().eq(nominate_chancelor)
                        },
                        false => {
                            true
                        },
                    }
                },
                false =>{
                    true
                }
            };
        require!(nomination_check,GameErrorCode::InvalidChancellorNominated);
        
        
        Ok(())
    }
    
}
