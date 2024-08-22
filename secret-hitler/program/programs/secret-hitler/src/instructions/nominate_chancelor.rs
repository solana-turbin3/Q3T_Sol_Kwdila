use anchor_lang::prelude::*;

use crate::enums::GameState;
use crate::state::nomination::Nomination;
use crate::state::{game_data::GameData, player_data::PlayerData};
use crate::GameErrorCode;

#[derive(Accounts)]
pub struct NominateChancellor<'info> {
    #[account(mut)]
    pub president: Signer<'info>,
    #[account(
        seeds = [
            game_data.key().to_bytes().as_ref(),
            president.key().to_bytes().as_ref(),
            ],
        bump = player_data.bump,
        constraint = player_data.is_active @GameErrorCode::InactivePlayer
    )]
    pub player_data: Account<'info, PlayerData>,
    #[account(
        init,
        payer=president,
        space= Nomination::INIT_SPACE,
        seeds =[
            b"chancellor_nomination",
            game_data.key().to_bytes().as_ref()
            ],
        bump
    )]
    pub nomination: Account<'info, Nomination>,
    #[account(
        seeds = [
            b"secret_hitler",
            game_data.host.to_bytes().as_ref()
            ],
        bump = game_data.bump,

        constraint = president.key().eq(
            game_data.players.get(game_data.current_president_index as usize).unwrap() // this is checked
        ) @GameErrorCode::PlayerNotInGame,

        constraint = GameState::ChancellorNomination == game_data.game_state @GameErrorCode::InvalidGameState,
    )]
    pub game_data: Account<'info, GameData>,
    pub system_program: Program<'info, System>,
}

impl<'info> NominateChancellor<'info> {
    pub fn nominated_chancellor(
        &mut self,
        nominated_chancellor_index: u8,
        bumps: NominateChancellorBumps,
    ) -> Result<()> {
        let game = &mut self.game_data;
        require!(
            nominated_chancellor_index < 10,
            GameErrorCode::PlayerNotInGame
        );

        let nominated_chancelor = game
            .players
            .get(nominated_chancellor_index as usize)
            .unwrap();

        let prev_president = game.previous_president_index;

        let nomination_check: bool = match game.previous_chancellor_index.is_some() {
            //check if nominated_chancellor is eligible
            true => {
                let prev_chancellor_index = game.previous_chancellor_index.unwrap();

                let mut result = game
                    .players
                    .get(prev_chancellor_index as usize)
                    .unwrap()
                    .eq(nominated_chancelor); //prev chancellor ineligible

                match game.player_count <= 5 {
                    true => result,
                    false => {
                        if prev_president.is_some() {
                            result &= game
                                .players
                                .get(prev_president.unwrap() as usize)
                                .unwrap()
                                .eq(nominated_chancelor) //prev president ineligible
                        }
                        result
                    }
                }
            }
            false => true, // no checks needed if this is the first chancellor
        };
        require!(
            nomination_check,
            GameErrorCode::IneligibleChancellorNominated
        );

        game.game_state = GameState::ChancellorVoting;

        self.nomination.init(
            nominated_chancellor_index,
            self.game_data.current_president_index,
            bumps.nomination,
        )?;

        Ok(())
    }
}
