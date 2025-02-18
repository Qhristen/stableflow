use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ConfigState {
    pub admin: Pubkey,
    pub treasury_fee: u8,
    pub bump: u8,
}
