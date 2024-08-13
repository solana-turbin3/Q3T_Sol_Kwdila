use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    #[account(mut)]
    pub taker: Signer<'info>,

    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(mut, associated_token::mint = mint_a, associated_token::authority = maker)]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(init_if_needed,payer = taker, associated_token::mint = mint_b, associated_token::authority = maker)]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(init_if_needed,payer = taker, associated_token::mint = mint_a, associated_token::authority = taker)]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, associated_token::mint = mint_b, associated_token::authority = taker)]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            b"escrow",
            escrow.maker.as_ref(),
            escrow.seed.to_le_bytes().as_ref()
        ],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}
impl<'info> Take<'info> {
    pub fn deposit_into_vault(&mut self) -> Result<()> {
        let accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);

        transfer_checked(ctx, self.escrow.recieve_amount, self.mint_a.decimals)?;

        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let escrow_seed = self.escrow.seed.to_le_bytes();
        let seeds = [
            b"escrow",
            self.escrow.maker.as_ref(),
            escrow_seed.as_ref(),
            &[self.escrow.bump],
        ];
        let signer_seeds: &[&[&[u8]]] = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer_checked(ctx, self.escrow.recieve_amount, self.mint_a.decimals)?;
        Ok(())
    }
    pub fn withdraw_and_close_vault(&mut self) -> Result<()> {
        let accounts = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);

        transfer_checked(ctx, self.vault.amount, self.mint_a.decimals)?;
        Ok(())
    }
}
