use anchor_lang::prelude::*;

use crate::state::Counter;

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"counter", user.key().as_ref()],
        bump = counter.bump,
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

impl<'info> Increment<'info> {
    pub fn increment(&mut self) -> Result<()> {
        self.counter.count = self.counter.count.saturating_add(1);

        Ok(())
    }
}