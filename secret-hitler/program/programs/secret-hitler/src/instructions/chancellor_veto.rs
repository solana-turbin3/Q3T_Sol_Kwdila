use anchor_lang::prelude::*;

use crate::{
    state::{GameData, PlayerData},
    GameErrorCode, GameState,
};

#[derive(Accounts)]
pub struct ChancellorVeto<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        seeds = [
            b"player_data",
            game_data.key().to_bytes().as_ref(),
            player.key().to_bytes().as_ref(),
            ],
        bump = player_data.bump,
    )]
    pub player_data: Account<'info, PlayerData>,
    #[account(
        mut,
        seeds = [
            b"secret_hitler",
            game_data.host.to_bytes().as_ref()
            ],
        bump = game_data.bump,

        // Chancellor needs to veto vote in LegistlativeChancellor state
        constraint = game_data.is_chancellor(player.key) && game_data.game_state == GameState::LegislativeChancellor @ GameErrorCode::ChancellorPolicyError,
    )]
    pub game_data: Account<'info, GameData>,
}

impl<'info> ChancellorVeto<'info> {
    pub fn veto(&mut self) -> Result<()> {
        self.game_data
            .next_turn(GameState::LegislativePresidentVeto)?;
        Ok(())
    }
}
