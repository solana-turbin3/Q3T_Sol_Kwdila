use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        mint_to, transfer_checked, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked,
    },
};

use crate::Config;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub provider: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds = [b"lp", config.key().as_ref()],
        bump=config.lp_bump,
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = provider
    )]
    pub provider_ata_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = provider
    )]
    pub provider_ata_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = provider,
        associated_token::mint = mint_lp,
        associated_token::authority = provider
    )]
    pub provider_ata_lp: InterfaceAccount<'info, TokenAccount>,

    #[account(
        associated_token::mint = mint_x,
        associated_token::authority = config
    )]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        associated_token::mint = mint_y,
        associated_token::authority = config
    )]
    pub vault_y: InterfaceAccount<'info, TokenAccount>,

    #[account(
        has_one=mint_x,
        has_one=mint_y,

        seeds=[
            b"config",
            provider.key().to_bytes().as_ref(),
            mint_x.key().to_bytes().as_ref(),
            mint_y.key().to_bytes().as_ref(),
            seed.to_le_bytes().as_ref(),
        ],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn deposit(&mut self, amount: u64, is_x: bool) -> Result<()> {
        let (mint, provider_ata, vault, decimals) = match is_x {
            true => (
                self.mint_x.to_account_info(),
                self.mint_x.to_account_info(),
                self.vault_x.to_account_info(),
                self.mint_x.decimals,
            ),
            false => (
                self.mint_y.to_account_info(),
                self.mint_y.to_account_info(),
                self.vault_y.to_account_info(),
                self.mint_y.decimals,
            ),
        };
        let accounts = TransferChecked {
            from: provider_ata,
            to: vault,
            mint: mint,
            authority: self.provider.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);

        transfer_checked(ctx, amount, decimals)?;
        Ok(())
    }
    pub fn mint_lp_token(&mut self, max_x: u64, max_y: u64) -> Result<()> {
        let amount = max_x
            .checked_mul(max_y)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        let accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.provider_ata_lp.to_account_info(),
            authority: self.config.to_account_info(),
        };
        let provider_key = self.provider.key().to_bytes();
        let mint_y = self.mint_y.key().to_bytes();
        let mint_x = self.mint_x.key().to_bytes();
        let seed = self.config.seed.to_le_bytes();
        let seeds = [
            b"config",
            provider_key.as_ref(),
            mint_x.as_ref(),
            mint_y.as_ref(),
            seed.as_ref(),
        ];
        let signer_seeds = &[&seeds[..]];
        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        mint_to(ctx, amount)?;
        Ok(())
    }
}
