use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserState {
    pub user: Pubkey,
    pub total_deposited_amount: u64,
    pub earned_yield: u64,
    pub bump: u8,
}