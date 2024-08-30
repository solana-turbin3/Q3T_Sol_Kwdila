use anchor_lang::prelude::*;

use crate::{
    constants::{FASCIST_VICTORY_POLICIES, LIBERAL_VICTORY_POLICIES},
    state::GameData,
    FascistBoard, GameErrorCode, GameState,
    PlayerCount::*,
    PolicyCard,
};

#[derive(Accounts)]
pub struct EnactPolicy<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        mut,
        seeds = [
            b"secret_hitler",
            game_data.host.to_bytes().as_ref()
            ],
        bump = game_data.bump,

        // president needs to enact policy in LegistlativePresident state
        constraint = game_data.is_president(player.key) && game_data.game_state == GameState::LegislativePresident @ GameErrorCode::PresidentPolicyError,
        // Chancellor needs to enact policy in LegistlativeChancellor state
        constraint = game_data.is_chancellor(player.key) && game_data.game_state == GameState::LegislativeChancellor @ GameErrorCode::ChancellorPolicyError,
    )]
    pub game_data: Account<'info, GameData>,
}

impl<'info> EnactPolicy<'info> {
    pub fn enact_policy(&mut self, policy: Option<PolicyCard>) -> Result<()> {
        let game = &mut self.game_data;

        let current_president_key = game
            .active_players
            .get(game.current_president_index as usize)
            .ok_or(GameErrorCode::PlayerNotInGame)?;
        let current_chancellor_key = game
            .active_players
            .get(
                game.current_chancellor_index
                    .ok_or(GameErrorCode::InvalidGameState)? as usize,
            )
            .ok_or(GameErrorCode::PlayerNotInGame)?;

        // Ensure player is either the current president or chancellor
        if ![current_president_key, current_chancellor_key].contains(&self.player.key) {
            return err!(GameErrorCode::PlayerNotInGovernment);
        }

        // Check game state for the president or chancellor
        match game.game_state {
            GameState::LegislativePresident => {
                require_keys_neq!(
                    self.player.key(),
                    *current_president_key,
                    GameErrorCode::PresidentPolicyError
                );
                game.next_turn(GameState::LegislativeChancellor)?;
            }
            GameState::LegislativeChancellor => {
                require_keys_neq!(
                    self.player.key(),
                    *current_chancellor_key,
                    GameErrorCode::ChancellorPolicyError
                );
                match policy.ok_or(GameErrorCode::ChancellorPolicyError)? {
                    PolicyCard::Fascist => game.fascist_policies_enacted += 1,
                    PolicyCard::Liberal => game.liberal_policies_enacted += 1,
                }

                if game.liberal_policies_enacted == LIBERAL_VICTORY_POLICIES {
                    game.game_state = GameState::LiberalVictoryPolicy;
                    return Ok(());
                }
                if game.fascist_policies_enacted == 0 {
                    game.next_president();
                    game.next_turn(GameState::ChancellorNomination)?;
                    return Ok(());
                }
                if game.fascist_policies_enacted == FASCIST_VICTORY_POLICIES {
                    game.game_state = GameState::FascistVictoryPolicy;
                    return Ok(());
                }

                //handle different gameboards for different player counts
                let fascist_board = match game
                    .start_player_count
                    .ok_or(GameErrorCode::InvalidGameState)?
                {
                    Five | Six => FascistBoard::FiveToSix,
                    Seven | Eight => FascistBoard::SevenToEight,
                    Nine | Ten => FascistBoard::NineToTen,
                };

                // Determine the presidential power state based on the board and enacted policies
                let presidential_power = match (fascist_board, game.fascist_policies_enacted) {
                    (FascistBoard::FiveToSix, 1 | 2) | (FascistBoard::SevenToEight, 1) => {
                        game.next_president();
                        game.next_turn(GameState::ChancellorNomination)?;
                        None
                    }
                    (FascistBoard::FiveToSix, 3) => Some(GameState::PresidentialPowerPeek),
                    (FascistBoard::FiveToSix, 4 | 5)
                    | (FascistBoard::SevenToEight, 4 | 5)
                    | (FascistBoard::NineToTen, 4 | 5) => {
                        Some(GameState::PresidentialPowerExecution)
                    }
                    (FascistBoard::SevenToEight, 2) | (FascistBoard::NineToTen, 1 | 2) => {
                        Some(GameState::PresidentialPowerInvestigate)
                    }
                    (FascistBoard::SevenToEight, 3) | (FascistBoard::NineToTen, 3) => {
                        Some(GameState::PresidentialPowerElection)
                    }
                    _ => None,
                };

                // Proceed to the next turn if a presidential power state is determined
                if let Some(state) = presidential_power {
                    game.next_turn(state)?;
                }
            }
            _ => return err!(GameErrorCode::InvalidGameState),
        }

        Ok(())
    }
}
