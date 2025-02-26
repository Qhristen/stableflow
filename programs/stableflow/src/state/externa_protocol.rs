use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ExternalProtocolState {
    pub allocated_funds: u64,        // Funds allocated to this protocol
    pub pool_id: Pubkey,         // External protocol's public key
    #[max_len(32)]
    pub name: String,         // External protocol's public key
    pub bump: u8,    
}