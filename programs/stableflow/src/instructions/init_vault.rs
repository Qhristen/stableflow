use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::VaultState;

#[derive(Accounts)]
#[instruction(seed: String)]
pub struct InitializeVault<'info>  {
    #[account(mut)]
    pub user: Signer<'info>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init, 
        payer = user, 
        seeds = [b"vault_state", user.key().as_ref(), seed.as_bytes()],
        space = 8 + VaultState::INIT_SPACE,
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        init,
        payer = user,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = vault_state,

    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeVault<'info> {
    pub fn init_vault(&mut self, seed: String, bumps: &InitializeVaultBumps ) -> Result<()> {
        self.vault_state.set_inner(VaultState{
            token_mint: self.token_mint.key(),
            total_deposited: 0,
            total_rewards: 0,
            seed,
            vault_bump: bumps.vault,
            state_bump: bumps.vault_state
        });
        
        Ok(())

    
    }
}