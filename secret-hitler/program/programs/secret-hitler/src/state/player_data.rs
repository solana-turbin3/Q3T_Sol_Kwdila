use anchor_lang::prelude::*;

#[account]
pub struct PlayerData {
    pub role: Option<[u8; 32]>, //hashed role
    pub is_in_game: bool,
    pub bump: u8,
}

impl Space for PlayerData {
    const INIT_SPACE: usize = 8 + 33 + 1;
}
