use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub maker: Pubkey,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub seed: u64,
    pub fee: u16, //Bases Fee
    pub bump: u8,
    pub lp_bump: u8,
    pub locked: bool,
}

impl Config {
    pub const LEN: usize = 8 + (32 * 3) + 8 + 16;

    pub fn init(
        &mut self,
        maker: Pubkey,
        mint_x: Pubkey,
        mint_y: Pubkey,
        seed: u64,
        fee: u16,
        bump: u8,
        lp_bump: u8,
        locked: bool,
    ) {
        self.maker = maker;
        self.mint_x = mint_x;
        self.mint_y = mint_y;
        self.seed = seed;
        self.fee = fee;
        self.bump = bump;
        self.lp_bump = lp_bump;
        self.locked = locked;
    }
}
