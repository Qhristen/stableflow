use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ConfigState {
    pub admin: Pubkey,
    pub total_deposits: u64,
    pub total_yield: u64,
    pub fee: u16, // Swap fee in basis points
    pub bump: u8,

}
