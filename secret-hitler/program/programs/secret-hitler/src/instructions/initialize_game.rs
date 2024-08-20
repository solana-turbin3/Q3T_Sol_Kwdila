use anchor_lang::{prelude::*, system_program::{Transfer,transfer}};

use crate::{state::{game_data::GameData, player_data::PlayerData}, GameErrorCode};

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(mut)]
    pub host: Signer<'info>,

    #[account(
        seeds = [
            b"deposit_vault",
            game_data.key().to_bytes().as_ref()
        ],
        bump
    )]
    pub deposit_vault: Option<SystemAccount<'info>>,

        #[account(
        seeds = [
            b"bet_vault",
            game_data.key().to_bytes().as_ref()
        ],
        bump
    )]
    pub bet_vault: Option<SystemAccount<'info>>,

    #[account(
        init,
        payer = host,
        space = PlayerData::INIT_SPACE,
        seeds = [            
            game_data.key().to_bytes().as_ref(),
            host.key().to_bytes().as_ref()
        ],
        bump
    )]
    pub player_data: Account<'info, PlayerData>,

    #[account(
        init,
        payer=host,
        space=GameData::INIT_SPACE,
        seeds = [
            b"secret_hitler",
            host.key().to_bytes().as_ref()
        ],
        bump,
        constraint = game_data.entry_deposit.is_some() == deposit_vault.is_some() @GameErrorCode::DepositVaultNotFound,
        constraint = game_data.bet_amount.is_some() == bet_vault.is_some() @GameErrorCode::BetVaultNotFound,
    )]
    pub game_data: Account<'info, GameData>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeGame<'info> {
    pub fn init_game(
        &mut self,
        max_players: u8,
        entry_deposit: Option<u64>,
        bet_amount: Option<u64>,
        bumps: InitializeGameBumps,
    ) -> Result<()> {
        require!(max_players >= 5,GameErrorCode::MinimumPlayersNotReached);
        
        self.game_data.init(
            self.host.key(),
            max_players,
            entry_deposit,
            bet_amount,
            bumps.game_data,
            bumps.deposit_vault,
            bumps.bet_vault
        );
            

        match self.game_data.entry_deposit {
            Some(deposit) => {
                let accounts = Transfer {
                    from: self.host.to_account_info(),
                    to: self.deposit_vault.as_ref().unwrap().to_account_info(), //this is checked in game_data account constraints
                };

                let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
                transfer(ctx, deposit)?
            },
            None => (),
        }

        match self.game_data.bet_amount {
            Some(deposit) => {
                let accounts = Transfer {
                    from: self.host.to_account_info(),
                    to: self.deposit_vault.as_ref().unwrap().to_account_info(), //this is checked in game_data account constraints
                };

                let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
                transfer(ctx, deposit)?
            },
            None => (),
        }

        self.player_data.set_inner(PlayerData {
            role: None,
            is_in_game: true,
            bump: bumps.player_data,
        });

        Ok(())
    }
}
