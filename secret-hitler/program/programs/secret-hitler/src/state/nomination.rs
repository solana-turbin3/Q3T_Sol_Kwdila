use anchor_lang::prelude::*;

#[account]
pub struct Nomination {
    pub nominee_index: u8,
    pub nein: u8,
    pub ja: u8,
    pub bump: u8,
}

impl Space for Nomination {
    const INIT_SPACE: usize = 8 + 8 + 1 * 3;
}

impl Nomination {
    pub fn init(&mut self, index: u8, bump: u8) {
        self.nominee_index = index;
        self.nein = 0;
        self.ja = 0;
        self.bump = bump;
    }
}
