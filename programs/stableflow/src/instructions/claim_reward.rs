use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{error::CustomError, UserState, VaultState};

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = vault_state,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(mut, has_one = user, has_one = vault)]
    pub user_account: Account<'info, UserState>,

    #[account(mut)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimReward<'info> {
    pub fn claim(&mut self) -> Result<()> {
        let user = &mut self.user_account;
        let vault_state = &mut self.vault_state;

        let rewards_amount = user.pending_rewards;
        require!(rewards_amount > 0, CustomError::NoRewards);


        // Calculate the seeds for the vault's PDA
        let vault_bump = &[vault_state.vault_bump][..];
        let vault_address = vault_state.key();
        let seeds = &[
            b"vault_state",
            vault_address.as_ref(),
            vault_bump,
        ];

        let signer_seeds = &[&seeds[..]];

        // Transfer tokens from vault to user
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user_token_account.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx: CpiContext<'_, '_, '_, '_, Transfer<'_>> = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, rewards_amount)?;

       // Update state
       user.pending_rewards = 0;
       vault_state.total_rewards = vault_state.total_rewards.checked_sub(rewards_amount)
           .ok_or(CustomError::Overflow)?;

        Ok(())
    }
}
