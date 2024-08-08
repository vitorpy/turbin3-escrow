use anchor_lang::prelude::*;
use anchor_spl::token_interface::*;
use anchor_spl::associated_token::AssociatedToken;

use crate::Escrow;

#[derive(Accounts)]
#[instruction(seed: u8)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref(), &seed.to_le_bytes()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    pub fn init_escrow(&mut self, seed: u64, receive: u64, bumps: &MakeBumps) -> Result<()> {
        self.escrow.set_inner(Escrow {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive: receive,
            bump: bumps.escrow,
        });
        
        Ok(())
    }
    
    pub fn deposit(&mut self, deposit: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info().clone();
        
        let cpi_account = TransferChecked {
            from: self.maker_ata.to_account_info().clone(),
            to: self.vault.to_account_info().clone(),
            authority: self.maker.to_account_info().clone(),
            mint: self.mint_a.to_account_info().clone(),
        };
        
        let cpi_ctxt = CpiContext::new(cpi_program, cpi_account);

        transfer_checked(cpi_ctxt, deposit, self.mint_a.decimals)?;

        Ok(())
    }
}