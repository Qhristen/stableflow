use crate::{ExternalProtocolState, VaultState};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount};

#[derive(Accounts)]
#[instruction(protocol_id: String)]
pub struct AddExternalProtocol<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(init, payer = user, space = 8 + ExternalProtocolState::INIT_SPACE)]
    pub protocol: Account<'info, ExternalProtocolState>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = vault_state,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        close = user,
        has_one = token_mint,
        seeds = [b"vault_state", user.key().as_ref(), vault_state.seed.as_bytes()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    /// CHECK: This is safe as we just store the pubkey
    pub protocol_id: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> AddExternalProtocol<'info> {
    pub fn add(&mut self, protocol_id: String) -> Result<()> {
        self.protocol.set_inner(ExternalProtocolState {
            allocated_funds: 0,
            protocol_id,
            vault: self.vault.key(),
        });
        Ok(())
    }
}
