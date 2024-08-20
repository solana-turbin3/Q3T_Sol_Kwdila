use anchor_lang::prelude::*;

use crate::state::{game_data::GameData, player_data::PlayerData};

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(
        mut,
        address=game_data.current_president
    )]
    pub president: Signer<'info>,
    #[account(
        seeds = [
            b"deposit",
            game_data.key().to_bytes().as_ref()
        ],
        bump = game_data.depo
    )]
    pub deposit_vault: SystemAccount<'info>,
    #[account(
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
        bump
    )]
    pub game_data: Account<'info, GameData>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeGame<'info> {
    pub fn nominate_chancellor(
        &mut self,
        max_players: u8,
        entry_deposit: Option<u64>,
        bet_amount: Option<u64>,
        bumps: InitializeGameBumps,
    ) -> Result<()> {
        todo!()
    }
}
