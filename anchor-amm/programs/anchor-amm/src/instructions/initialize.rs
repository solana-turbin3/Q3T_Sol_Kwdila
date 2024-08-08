use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::Config;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,

    #[account(init,payer=maker,space= Config::SPACE,seeds=[b"config",maker.key().to_bytes().as_ref(),mint_x.key().to_bytes().as_ref(),mint_y.key().to_bytes().as_ref()],bump)]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize_config(&mut self) -> Result<()> {
        Ok(())
    }
}
