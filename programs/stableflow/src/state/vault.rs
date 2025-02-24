use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub total_deposited: u64,       // Total funds deposited in the vault
    pub total_rewards: u64,          // Total rewards accumulated
    pub token_mint: Pubkey,
    pub vault_bump: u8,                    // PDA bump for the vault
    pub state_bump: u8,                    // PDA bump for the vault
    #[max_len(32)]
    pub seed: String,                 
    // pub external_protocols: Vec<Pubkey>, // List of external protocols
}