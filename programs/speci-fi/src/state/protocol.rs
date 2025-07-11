use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Protocol {
    pub authority: Pubkey,
    pub fee_bps: u64,
    pub total_loans: u64,
    pub total_volume: u64,
    pub is_paused: bool,
    pub bump: u8,
}
