use anchor_lang::prelude::*;

use crate::{GameState, PlayerCount};

#[account]
pub struct GameData {
    // these are set and do not change after the game starts
    pub host: Pubkey,
    pub turn_duration: i64,
    pub max_players: u8,
    pub entry_deposit: Option<u64>, // returned to everyone completing the game
    pub bet_amount: Option<u64>,    // devided between winners
    pub start_player_count: Option<PlayerCount>,

    // these change during the game
    pub active_players: Vec<Pubkey>,
    pub eliminated_players: Vec<Pubkey>,
    pub turn_started_at: Option<i64>,
    pub game_state: GameState,
    pub fascist_policies_enacted: u8,
    pub liberal_policies_enacted: u8,
    pub failed_elections: u8,

    pub current_president_index: u64,
    pub previous_president_index: Option<u64>,
    pub current_chancellor_index: Option<u64>,
    pub is_special_election: bool,
    pub previous_chancellor_index: Option<u64>,

    //bumps
    pub bump: u8,
    pub deposit_vault_bump: Option<u8>,
    pub bet_vault_bump: Option<u8>,

}

impl Space for GameData {
    const INIT_SPACE: usize = 
    8               // anchor descriminator
    + 32            // Pubkey
    + 4 + 32 * 10   // Vec<Pubkey>
    + 4 + 32 * 2    // Vec<Pubkey>
    + 9 * 6         // Option<u64/i64>
    + 2 * 2         // Option<u8>
    + 8             // u64
    + 1 * 5         // u8
    + 1             // GameState
    + 1             // PlayerCount
    + 1             // bool
    ;
}

impl GameData {
    pub fn init(
        &mut self,
        host: Pubkey,
        max_players: u8,
        turn_duration:i64,
        entry_deposit: Option<u64>,
        bet_amount: Option<u64>,
        game_data_bump: u8,
        deposit_vault_bump: Option<u8>,
        bet_vault_bump: Option<u8>,
    ) -> Result<()>{
        self.host = host;

        self.is_special_election = false;
        self.current_president_index = 4;
        self.current_chancellor_index = None;
        self.previous_president_index = None;
        self.previous_chancellor_index = None;

        self.turn_duration = turn_duration;
        self.max_players = max_players;
        self.active_players = vec![host];
        self.eliminated_players = Vec::new();
        self.turn_started_at = None;
        self.start_player_count = None;

        self.entry_deposit = entry_deposit;
        self.bet_amount = bet_amount;

        self.game_state = GameState::Setup;
        self.fascist_policies_enacted = 0;
        self.liberal_policies_enacted = 0;
        self.failed_elections = 0;

        self.bump = game_data_bump;
        self.deposit_vault_bump = deposit_vault_bump;
        self.bet_vault_bump = bet_vault_bump;

        Ok(())
    }

    pub fn next_turn(&mut self, state: GameState) -> Result<()>{
        let clock = Clock::get()?;
        self.turn_started_at = Some(clock.unix_timestamp);
        self.game_state = state;
        Ok(())
    }

    pub fn next_president(&mut self) {
        // if there was a special election in the past, then return to normal flow of presidents
        if self.is_special_election {
            self.is_special_election = false;
            self.current_president_index = (self.previous_president_index.unwrap() + 1) % self.active_players.len() as u64;
            return;
        }
        self.previous_president_index = Some(self.current_president_index);
        self.current_president_index = (self.current_president_index + 1) % self.active_players.len() as u64;
    }

    pub fn special_election(&mut self, new_president:u64) {
        self.is_special_election = true;
        self.previous_president_index = Some(self.current_president_index);
        self.current_president_index = new_president;
    }

    pub fn is_in_game(&self, player_key: &Pubkey) -> bool { 
        self.active_players.contains(player_key)
    }

    pub fn is_president(&self, player_key: &Pubkey) -> bool {
        self.active_players
        .get(self.current_president_index as usize)
        .map_or(false, |current_president| {
            player_key == current_president
        })
    }
    pub fn is_chancellor(&self, player_key: &Pubkey) -> bool {
        self.current_chancellor_index
        .and_then(|index| self.active_players.get(index as usize))
        .map_or(false, |chancellor| player_key == chancellor)
    }
}

