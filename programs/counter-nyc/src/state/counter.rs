use anchor_lang::prelude::*;

#[account]
pub struct Counter {
    pub count: u64,
    pub bump: u8,
}

impl Space for Counter {
    const INIT_SPACE: usize =  8 + 8 + 1;
}