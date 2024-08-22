use anchor_lang::prelude::*;

#[account]
pub struct Nomination {
    pub voters_index: Vec<u8>,
    pub nominee_index: u8,
    pub nein: u8,
    pub ja: u8,
    pub bump: u8,
    pub created_at: i64,
}

impl Space for Nomination {
    const INIT_SPACE: usize = 8 + (4 + 10) + (1 * 4) + 8;
}

impl Nomination {
    pub fn init(&mut self, index: u8, president_index: u8, bump: u8) -> Result<()> {
        self.voters_index.push(president_index);
        self.nominee_index = index;
        self.nein = 0;
        self.ja = 1; // it is assumed the president votes ja by nominating
        self.bump = bump;
        self.created_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
}
