use anchor_lang::prelude::*;

use crate::{
    state::{GameData, PlayerData},
    GameErrorCode, GameState, PolicyCard,
};

#[derive(Accounts)]
pub struct EnactPolicy<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        seeds = [
            game_data.key().to_bytes().as_ref(),
            player.key().to_bytes().as_ref(),
            ],
        bump = player_data.bump,

        // posibly redundant, because goverment is already chosen from active players
        constraint = !player_data.is_eliminated @GameErrorCode::EiminatedPlayer 
    )]
    pub player_data: Account<'info, PlayerData>,
    #[account(
        seeds = [
            b"secret_hitler",
            game_data.host.to_bytes().as_ref()
            ],
        bump = game_data.bump,

        // president needs to enact policy in LegistlativePresident state
        constraint = player.key().eq(
            game_data.active_players.get(game_data.current_president_index as usize).ok_or(GameErrorCode::PlayerNotInGame)?
        ) && GameState::LegislativePresident == game_data.game_state @GameErrorCode::PresidentPolicyError,

        // Chancellor needs to enact policy in LegistlativeChancellor state
        constraint = player.key().eq(
            game_data.active_players.get(game_data.current_chancellor_index.ok_or(GameErrorCode::InvalidGameState)? as usize).ok_or(GameErrorCode::PlayerNotInGame)?
        ) && GameState::LegislativeChancellor == game_data.game_state @GameErrorCode::ChancellorPolicyError,

    )]
    pub game_data: Account<'info, GameData>,
}

impl<'info> EnactPolicy<'info> {
    pub fn enact_policy(&mut self, policy: Option<PolicyCard>) -> Result<()> {
        let game = &mut self.game_data;

        let current_president_key = game.active_players
            .get(game.current_president_index as usize)
            .ok_or(GameErrorCode::PlayerNotInGame)?;
        let current_chancellor_key = game.active_players
            .get(game.current_chancellor_index
            .ok_or(GameErrorCode::InvalidGameState)? as usize)
            .ok_or(GameErrorCode::PlayerNotInGame)?;

        // Ensure player is either the current president or chancellor
        if ![current_president_key, current_chancellor_key].contains(&self.player.key) {
            return err!(GameErrorCode::PlayerNotInGovernment);
        }

        // Check game state for the president or chancellor
        match game.game_state {
            GameState::LegislativePresident => {
                require_keys_neq!(self.player.key(), *current_president_key, GameErrorCode::PresidentPolicyError);
                game.next_turn(GameState::LegislativeChancellor)?;
            }
            GameState::LegislativeChancellor => {
                require_keys_neq!(self.player.key(), *current_chancellor_key, GameErrorCode::ChancellorPolicyError);
                match policy.ok_or(GameErrorCode::ChancellorPolicyError)? {
                    PolicyCard::Fascist => game.fascist_policies_enacted += 1,
                    PolicyCard::Liberal => game.liberal_policies_enacted += 1,
                }

                // self.check_win_conditions()

                //handle different gameboards for different player counts

                match game.fascist_policies_enacted {
                    0..=2 => {
                        game.next_president();
                        game.next_turn(GameState::ChancellorNomination)?;
                    }
                }
            }
            _ => return err!(GameErrorCode::InvalidGameState),
        }

        Ok(())
    }
    pub fn executive_action(&mut self) -> Result<()>{
        Ok(())
    }
}
