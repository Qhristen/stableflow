use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserState {
    pub user: Pubkey,                // User's public key
    pub deposited_amount: u64,       // Amount deposited by the user
    pub pending_rewards: u64,        // Rewards pending to be claimed
    pub vault: Pubkey,                // Associated vault
    pub bump: u8,                     // PDA bump for the user account
}