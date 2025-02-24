use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ExternalProtocolState {
    pub allocated_funds: u64,        // Funds allocated to this protocol
    pub vault: Pubkey,
    #[max_len(32)]
    pub protocol_id: String,         // External protocol's public key
}