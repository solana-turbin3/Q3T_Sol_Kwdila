use anchor_lang::prelude::*;

use crate::enums::GameState;
use crate::state::game_data::GameData;
use crate::GameErrorCode;

#[derive(Accounts)]
pub struct PresidentPower<'info> {
    #[account(mut)]
    pub president: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"secret_hitler",
            game_data.host.to_bytes().as_ref()
            ],
        bump = game_data.bump,

        constraint = game_data.is_president(president.key) @GameErrorCode::PresidentRoleRequired,
    )]
    pub game_data: Account<'info, GameData>,
}

impl<'info> PresidentPower<'info> {
    pub fn activate_president_veto(&mut self, targeted_player_index: u64) -> Result<()> {
        let game = &mut self.game_data;
        let target_player_key = game
            .active_players
            .get(targeted_player_index as usize)
            .ok_or(GameErrorCode::PlayerNotInGame)?;

        match game.game_state {
            GameState::PresidentialPowerElection => {
                game.special_election(targeted_player_index);
                game.next_turn(GameState::ChancellorNomination)?;
            }
            GameState::PresidentialPowerPeek => {
                //                                                  //
                //              make request to server              //
                //                                                  //
                game.next_president();
                game.next_turn(GameState::ChancellorNomination)?;
            }
            GameState::PresidentialPowerInvestigate => {
                //                                                  //
                //              make request to server              //
                //                                                  //
                game.next_president();
                game.next_turn(GameState::ChancellorNomination)?;
            }
            GameState::PresidentialPowerExecution => {
                let player = *target_player_key;
                game.eliminated_players.push(player);
                game.active_players.remove(targeted_player_index as usize);
                game.next_president();
                game.next_turn(GameState::ChancellorNomination)?;
            }
            _ => return err!(GameErrorCode::InvalidGameState),
        }

        Ok(())
    }
}
