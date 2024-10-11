use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::Escrow;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(mut, associated_token::mint = mint_a, associated_token::authority = maker)]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [
            b"escrow",
            maker.key().as_ref(),
            seed.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init,
        seeds=[
            b"escrow_vault",
            escrow.key().to_bytes().as_ref(),
        ],
        payer =maker,
        token::authority = escrow,
        token::mint=mint_a,
        bump
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}
impl<'info> Make<'info> {
    pub fn initialize_escrow(
        &mut self,
        seed: u64,
        recieve_amount: u64,
        duration: u64,
        is_mutable: bool,
        bumps: MakeBumps,
    ) -> Result<()> {
        self.escrow.set_inner(Escrow {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            recieve_amount,
            expiry: 0,
            is_mutable,
            escrow_bump: bumps.escrow,
            vault_bump: bumps.vault,
        });
        self.escrow.set_expiry_relative(duration)?;
        Ok(())
    }
    pub fn deposit_into_escrow(&mut self) -> Result<()> {
        let accounts = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);

        transfer_checked(ctx, self.escrow.recieve_amount, self.mint_a.decimals)?;
        Ok(())
    }
}
