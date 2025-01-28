use anchor_lang::prelude::*;

declare_id!("By17PjZkiaeJJnzpbM1nmauhdFZGoPjcPxpXfUF5uYM4");

mod state;
mod instructions;

use instructions::*;

#[program]
pub mod counter_nyc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;

        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.increment()?;

        Ok(())
    }
}
