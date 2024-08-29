use anchor_lang::prelude::*;

use crate::enums::GameState;
use crate::state::game_data::GameData;
use crate::state::nomination::Nomination;
use crate::GameErrorCode;

#[derive(Accounts)]
pub struct NominateChancellor<'info> {
    #[account(mut)]
    pub president: Signer<'info>,
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

        constraint = game_data.is_president(president.key) @GameErrorCode::PresidentRoleRequired,
        constraint = GameState::ChancellorNomination == game_data.game_state @GameErrorCode::InvalidGameState,
    )]
    pub game_data: Account<'info, GameData>,
    pub system_program: Program<'info, System>,
}

impl<'info> NominateChancellor<'info> {
    pub fn nominated_chancellor(
        &mut self,
        nominated_chancellor_index: usize,
        bumps: NominateChancellorBumps,
    ) -> Result<()> {
        let game = &mut self.game_data;
        require!(
            nominated_chancellor_index < 10,
            GameErrorCode::PlayerNotInGame
        );

        let nominated_chancelor = game
            .active_players
            .get(nominated_chancellor_index as usize)
            .ok_or(GameErrorCode::PlayerNotInGame)?;

        let prev_president = game.previous_president_index;

        let nomination_check: bool = match game.previous_chancellor_index.is_some() {
            //check if nominated_chancellor is eligible
            true => {
                let prev_chancellor_index = game
                    .previous_chancellor_index
                    .ok_or(GameErrorCode::PrevChancellorNotFound)?;

                let mut result = game
                    .active_players
                    .get(prev_chancellor_index as usize)
                    .ok_or(GameErrorCode::PlayerNotInGame)?
                    .eq(nominated_chancelor); //prev chancellor ineligible

                match game.active_players.len() <= 5 {
                    true => result,
                    false => {
                        if prev_president.is_some() {
                            result &= game
                                .active_players
                                .get(prev_president.ok_or(GameErrorCode::PrevPresidentNotFound)?
                                    as usize)
                                .ok_or(GameErrorCode::PlayerNotInGame)?
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

        game.next_turn(GameState::ChancellorVoting)?;

        self.nomination.voters_index = vec![self.game_data.current_president_index];
        self.nomination.nominee_index = nominated_chancellor_index as u64;
        self.nomination.nein = 0;
        self.nomination.ja = 1; // it is assumed the president voted ja by nominating
        self.nomination.bump = bumps.nomination;

        Ok(())
    }
}
