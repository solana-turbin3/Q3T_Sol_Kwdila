use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::state::{game_data::GameData, player_data::PlayerData};
use crate::{enums::GameState, errors::GameErrorCode};

#[derive(Accounts)]
pub struct LeaveGame<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        mut,
        close=player,
        seeds = [
            game_data.key().to_bytes().as_ref(),
            player.key().to_bytes().as_ref()
        ],
        bump = player_data.bump,
        constraint = player_data.is_in_game @GameErrorCode::PlayerNotInGame
    )]
    pub player_data: Account<'info, PlayerData>,
    #[account(
        seeds= [
            b"deposit",
            game_data.key().to_bytes().as_ref()
        ],
        bump = game_data.deposit_vault_bump.unwrap(),
    )]
    pub deposit_vault: Option<SystemAccount<'info>>,
    #[account(
        seeds= [
        b"deposit_vault",
        game_data.key().to_bytes().as_ref()
    ],
        bump = game_data.bet_vault_bump.unwrap(),
    )]
    pub bet_vault: Option<SystemAccount<'info>>,
    #[account(
        mut,
        seeds = [
            b"secret_hitler",
            game_data.host.to_bytes().as_ref(),
        ],
        bump = game_data.bump,
        constraint = game_data.game_state == GameState::Setup @GameErrorCode::InvalidGameState,
        constraint = game_data.players.contains(player.key) @GameErrorCode::PlayerNotInGame,
        constraint = game_data.host.ne(player.key) @GameErrorCode::HostPlayerLeaving,
        constraint = game_data.entry_deposit.is_some() == deposit_vault.is_some() @GameErrorCode::DepositVaultNotFound,
        constraint = game_data.bet_amount.is_some() == bet_vault.is_some() @GameErrorCode::BetVaultNotFound,
    )]
    pub game_data: Account<'info, GameData>,
    pub system_program: Program<'info, System>,
}

impl<'info> LeaveGame<'info> {
    pub fn remove_player(&mut self) -> Result<()> {
        match self.game_data.entry_deposit {
            Some(amount) => {
                let accounts = Transfer {
                    from: self.player.to_account_info(),
                    to: self.deposit_vault.as_ref().unwrap().to_account_info(), //this is checked in game_data account constraints
                };

                let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);

                transfer(ctx, amount)?
            }
            None => (),
        }

        match self.game_data.bet_amount {
            Some(amount) => {
                let accounts = Transfer {
                    from: self.player.to_account_info(),
                    to: self.bet_vault.as_ref().unwrap().to_account_info(), //this is checked in game_data account constraints
                };

                let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);

                transfer(ctx, amount)?
            }
            None => (),
        }

        let index = self
            .game_data
            .players
            .iter()
            .position(|player| player == self.player.key)
            .unwrap(); // this is checked in the game_data account constraints

        self.game_data.players.swap_remove(index);

        self.game_data.player_count -= 1;

        Ok(())
    }
}
