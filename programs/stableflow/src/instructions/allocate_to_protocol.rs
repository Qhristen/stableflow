use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{error::CustomError, ExternalProtocolState, VaultState};

#[derive(Accounts)]
pub struct AllocateToExternalProtocol<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"external_protocol", protocol.pool_id.to_bytes().as_ref(), protocol.name.as_bytes()],
        bump
    )]
    pub protocol: Account<'info, ExternalProtocolState>,

    #[account(mut)]
    pub protocol_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = vault_state,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = user,
        has_one = token_mint,
        seeds = [b"vault_state", user.key().as_ref(), vault_state.seed.as_bytes()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> AllocateToExternalProtocol<'info> {
    pub fn allocate(&mut self, amount: u64) -> Result<()> {
        let vault_state = &mut self.vault_state;
        let protocol = &mut self.protocol;

        require!(
            amount <= vault_state.total_deposited,
            CustomError::InsufficientBalance
        );

        // Transfer tokens from vault to protocol
        let vault_bump = &[vault_state.vault_bump][..];
        let vault_address = vault_state.key();
        let seeds = &[b"vault_state", vault_address.as_ref(), vault_bump];

        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.protocol_token_account.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, amount)?;

        protocol.allocated_funds = protocol
            .allocated_funds
            .checked_add(amount)
            .ok_or(CustomError::Overflow)?;

        Ok(())
    }
}
