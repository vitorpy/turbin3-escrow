use anchor_lang::prelude::*;

declare_id!("5bVZtnMMGVk1E3TbtHbDRWUCftxRXaqzSZCFMYmrHJ5S");

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Make>, seed: u64, send: u64, receive: u64) -> Result<()> {
        ctx.accounts.init_escrow(
            seed,
            receive,
            &ctx.bumps,
        )?;
        ctx.accounts.deposit(send)?;

        Ok(())
    }
}
