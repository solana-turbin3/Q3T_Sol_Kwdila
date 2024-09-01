use anchor_lang::prelude::*;

#[account]
pub struct Nomination {
    pub voters_index: Vec<u64>,
    pub nominee_index: u64,
    pub nein: u8,
    pub ja: u8,
    pub bump: u8,
}

impl Space for Nomination {
    const INIT_SPACE: usize = 8 + 8 + (4 + 10) + (1 * 4);
}

impl Nomination {
    pub fn init(&mut self, index: u64, president_index: u64, bump: u8) {
        self.voters_index = vec![president_index];
        self.nominee_index = index;
        self.nein = 0;
        self.ja = 1; // it is assumed the president votes ja by nominating
        self.bump = bump;
    }
}
