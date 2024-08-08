use anchor_lang::prelude::*;
use anchor_spl::token_interface::*;
use anchor_spl::associated_token::AssociatedToken;

use crate::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    pub maker: SystemAccount<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        close = taker,
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [b"escrow", maker.key().as_ref(), &escrow.seed.to_le_bytes()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    pub fn pay(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info().clone();

        let cpi_accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info().clone(),
            to: self.taker_ata_a.to_account_info().clone(),
            authority: self.taker.to_account_info().clone(),
            mint: self.mint_b.to_account_info().clone(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, self.escrow.receive, self.mint_b.decimals)?;

        Ok(())
    }

    pub fn withdraw(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info().clone();

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info().clone(),
            to: self.taker_ata_b.to_account_info().clone(),
            authority: self.escrow.to_account_info().clone(),
            mint: self.mint_a.to_account_info().clone(),
        };

        let signer_seeds: &[&[&[u8]]] = &[&[&b"escrow"[..], self.maker.to_account_info().key.as_ref(), &self.escrow.seed.to_le_bytes()[..], &[self.escrow.bump]]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program.clone(), cpi_accounts, signer_seeds);
        
        transfer_checked(cpi_ctx, self.escrow.receive, self.mint_a.decimals)?;
        
        let close_accounts = CloseAccount {
            account: self.vault.to_account_info().clone(),
            destination: self.maker.to_account_info().clone(),
            authority: self.escrow.to_account_info().clone(),
        };

        let close_context = CpiContext::new_with_signer(cpi_program.clone(), close_accounts, signer_seeds);
        
        close_account(close_context)?;

        Ok(())
    }
}