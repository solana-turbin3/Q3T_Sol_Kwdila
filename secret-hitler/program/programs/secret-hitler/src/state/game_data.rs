use anchor_lang::prelude::*;

use crate::{FascistBoard, GameErrorCode, GameState, PlayerCount};

#[account]
pub struct GameData {
    // these are set and do not change after the game starts
    pub host: Pubkey,
    pub turn_duration: i64,
    pub max_players: u8,
    pub entry_deposit: Option<u64>, // returned to everyone completing the game
    pub bet_amount: Option<u64>,    // devided between winners
    pub start_player_count: Option<PlayerCount>,
    pub all_starting_players: Vec<Pubkey>,

    // these change during the game
    pub active_players: Vec<Pubkey>,
    pub eliminated_players: Vec<Pubkey>,
    pub turn_started_at: Option<i64>,
    pub game_state: GameState,
    pub fascist_policies_enacted: u8,
    pub liberal_policies_enacted: u8,
    pub failed_elections: u8,

    pub is_special_election: bool,
    pub current_president: Pubkey,
    pub previous_president: Option<Pubkey>,
    pub current_chancellor: Option<Pubkey>,
    pub previous_chancellor: Option<Pubkey>,

    //bumps
    pub bump: u8,
    pub deposit_vault_bump: Option<u8>,
    pub bet_vault_bump: Option<u8>,

}

impl Space for GameData {
    const INIT_SPACE: usize = 
    8               // anchor descriminator
    + 32 * 2        // Pubkey
    + 33 * 3        // Option<Pubkey>
    + 4 + 32 * 10   // Vec<Pubkey>
    + 4 + 32 * 10   // Vec<Pubkey>
    + 4 + 32 * 2    // Vec<Pubkey>
    + 9 * 3         // Option<u64/i64>
    + 2 * 2         // Option<u8>
    + 8             // i64
    + 1 * 5         // u8
    + 1             // GameState
    + 2             // Option<PlayerCount>
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
        self.current_president = self.active_players.get(3).unwrap().clone();
        self.current_chancellor = None;
        self.previous_president = None;
        self.previous_chancellor = None;

        self.turn_duration = turn_duration;
        self.max_players = max_players;
        self.active_players = vec![host];
        self.eliminated_players = Vec::new();
        self.turn_started_at = None;
        self.start_player_count = None;
        self.all_starting_players = Vec::new();

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
        self.previous_chancellor = self.current_chancellor;
        self.current_chancellor = None;
        // if there was a special election in the past, then return to normal flow of presidents
        if self.is_special_election {
            self.is_special_election = false;
            let next_president_index = self.active_players.iter().position(|key| key == &self.previous_president.unwrap()).unwrap() + 1 % self.active_players.len(); 
            self.current_president = self.active_players.get(next_president_index).unwrap().clone();
            return;
        }
        self.previous_president = Some(self.current_president);
        let next_president_index = self.active_players.iter().position(|key| key == &self.current_chancellor.unwrap()).unwrap() + 1 % self.active_players.len(); 
        self.current_president = self.active_players.get(next_president_index).unwrap().clone();
    }

    pub fn special_election(&mut self, new_president:Pubkey) {
        self.is_special_election = true;
        self.previous_president = Some(self.current_president);
        self.current_president = new_president;
    }

    pub fn is_in_game(&self, player_key: &Pubkey) -> bool { 
        self.active_players.contains(player_key)
    }

    pub fn is_president(&self, player_key: Pubkey) -> bool {
        player_key == self.current_president
    }    

    pub fn is_chancellor(&self, player_key: Pubkey) -> bool {
        self.current_chancellor
        .map_or(false, |chancellor| player_key == chancellor)
    }

    pub fn get_fascist_board(&self) -> Result<FascistBoard> {
        match self.start_player_count.ok_or(GameErrorCode::StartPlayerCountNotFound)? {
            PlayerCount::Five | PlayerCount::Six => Ok(FascistBoard::FiveToSix),
            PlayerCount::Seven | PlayerCount::Eight => Ok(FascistBoard::SevenToEight),
            PlayerCount::Nine | PlayerCount::Ten => Ok(FascistBoard::NineToTen),
        }
    }

    pub fn get_presidential_power_state(&self, fascist_board: FascistBoard) -> Option<GameState> {
         match (fascist_board, self.fascist_policies_enacted) {
                    (FascistBoard::FiveToSix, 1 | 2) | (FascistBoard::SevenToEight, 1) => {
                        None
                    },
                    (FascistBoard::FiveToSix, 3) => Some(GameState::PresidentialPowerPeek),
                    (FascistBoard::FiveToSix, 4 | 5)
                    | (FascistBoard::SevenToEight, 4 | 5)
                    | (FascistBoard::NineToTen, 4 | 5) => {
                        Some(GameState::PresidentialPowerExecution)
                    },
                    (FascistBoard::SevenToEight, 2) | (FascistBoard::NineToTen, 1 | 2) => {
                        Some(GameState::PresidentialPowerInvestigate)
                    },
                    (FascistBoard::SevenToEight, 3) | (FascistBoard::NineToTen, 3) => {
                        Some(GameState::PresidentialPowerElection)
                    },
                    _ => None,
         }
    }
}


