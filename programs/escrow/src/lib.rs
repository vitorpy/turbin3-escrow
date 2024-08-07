use anchor_lang::prelude::*;

declare_id!("5bVZtnMMGVk1E3TbtHbDRWUCftxRXaqzSZCFMYmrHJ5S");

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
