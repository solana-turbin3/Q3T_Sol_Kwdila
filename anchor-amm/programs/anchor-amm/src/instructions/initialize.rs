use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::Config;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        seeds = [b"lp", config.key.as_ref()],
        payer = maker,
        bump,
        mint::decimals = 6,
        mint::authority = config
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer=maker,
        space= Config::LEN,
        seeds=[
            b"config",
            maker.key().to_bytes().as_ref(),
            mint_x.key().to_bytes().as_ref(),
            mint_y.key().to_bytes().as_ref(),
            seed.to_le_bytes().as_ref(),
            ],
            bump
        )]
    pub config: Account<'info, Config>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize_config(&mut self, seed: u64, fee: u16, bumps: InitializeBumps) -> Result<()> {
        self.config.init(
            self.maker.key(),
            self.mint_x.key(),
            self.mint_y.key(),
            seed,
            fee,
            bumps.config,
        );
        Ok(())
    }
}
