use anchor_lang::prelude::*;

use crate::{constants::{NUM_FASCIST_POLICIES,NUM_LIBERAL_POLICIES}, enums::GameState};

#[account]
pub struct GameData {
    pub host: Pubkey,

    pub current_president_index: Option<usize>,
    pub current_chancellor_index: Option<usize>,
    pub previous_president_index: Option<usize>,
    pub previous_chancellor_index: Option<usize>,

    pub max_players: u8,
    pub player_count: u8,
    pub players: Vec<Pubkey>,

    pub entry_deposit: Option<u64>, // will be returned to everyone completing the game
    pub bet_amount: Option<u64>,    // will be devided between winners

    pub game_state: GameState,
    pub libral_cards_left: u8,
    pub fascist_cards_left: u8,

    pub bump: u8,
    pub deposit_vault_bump: Option<u8>,
    pub bet_vault_bump: Option<u8>,

}

impl Space for GameData {
    const INIT_SPACE: usize = 
    8               // anchor descriminator
    + 32            // pubkey
    + 9 * 4         // Option<usize>
    + 1 * 3         // u8
    + 4 + 32 * 10   // Vec<Pubkey>
    + 9 * 2         // Option<u64>
    + 1             // GameState
    + 2 * 2         // Option<u8>
    ;
}

impl GameData {
    pub fn init(
        &mut self,
        host: Pubkey,
        max_players: u8,
        entry_deposit: Option<u64>,
        bet_amount: Option<u64>,
        game_data_bump: u8,
        deposit_vault_bump: Option<u8>,
        bet_vault_bump: Option<u8>,
    ) {
        self.host = host;

        self.current_president_index = None;
        self.current_chancellor_index = None;
        self.previous_president_index = None;
        self.previous_chancellor_index = None;

        self.max_players = max_players;
        self.player_count = 1;
        self.players = vec![host];

        self.entry_deposit = entry_deposit;
        self.bet_amount = bet_amount;

        self.game_state = GameState::Setup;
        self.libral_cards_left = 

        self.bump = game_data_bump;
        self.deposit_vault_bump = deposit_vault_bump;
        self.bet_vault_bump = bet_vault_bump;
    }
}

