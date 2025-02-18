use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProtocolState {
    pub protocol_id: Pubkey,      // Public key of the protocol
    pub protocol_type: String,    // Type of protocol (e.g., "Saber", "Raydium")
    pub is_active: bool,          // Whether the protocol is active
}