use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
    TransferChecked,
};

use crate::{error::EscrowError, Escrow};

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(mut, associated_token::mint = escrow.mint_a, associated_token::authority = maker)]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds=[
            b"escrow_vault",
            escrow.key().to_bytes().as_ref(),
        ],
        token::authority = escrow,
        token::mint=escrow.mint_a,
        bump = escrow.vault_bump
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        has_one = maker,
        // make sure the supplied mint account matches the mint used to make the escrow
        has_one = mint_a @EscrowError::MintMismatch,
        seeds = [b"escrow", maker.key.as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.escrow_bump,
        close = maker,
    )]
    pub escrow: Account<'info, Escrow>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Cancel<'info> {
    // return the funds back to the maker
    pub fn withdraw_from_vault(&mut self) -> Result<()> {
        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.maker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let maker_key = self.maker.key();
        let escrow_seed = self.escrow.seed.to_le_bytes();
        let signer_seeds = &[
            b"escrow",
            maker_key.as_ref(),
            escrow_seed.as_ref(),
            &[self.escrow.escrow_bump],
        ];
        let binding = [&signer_seeds[..]];

        let ctx =
            CpiContext::new_with_signer(self.token_program.to_account_info(), accounts, &binding);

        transfer_checked(ctx, self.escrow.recieve_amount, self.mint_a.decimals)?;
        Ok(())
    }
    pub fn close_vault(&self) -> Result<()> {
        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        // use binding to prevent temporary value from being dropped
        let maker_key = self.maker.key();
        // use binding to prevent temporary value from being dropped
        let escrow_seed = self.escrow.seed.to_le_bytes();

        let signer_seeds = &[
            b"escrow",
            maker_key.as_ref(),
            escrow_seed.as_ref(),
            &[self.escrow.escrow_bump],
        ];
        let binding = [&signer_seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            &binding,
        );

        close_account(ctx)
    }
}
