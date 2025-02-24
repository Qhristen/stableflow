use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::{error::CustomError, UserState, VaultState};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        close = user,
        has_one = token_mint,
        seeds = [b"vault_state", user.key().as_ref(), vault_state.seed.as_bytes()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub user_account: Account<'info, UserState>,

    #[account(mut)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let vault_state = &mut self.vault_state.clone();
        let user_account = &mut self.user_account;

        require!(
            amount <= user_account.deposited_amount,
            CustomError::InsufficientBalance
        );

        // Calculate the seeds for the vault's PDA
        // let vault_address = vault_state.key();
        let user_key = self.user.key();
        let seeds = &[
            b"vault_state".as_ref(),
            user_key.as_ref(),
            &self.vault_state.seed.as_bytes(),
            &[self.vault_state.state_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        // Transfer tokens from vault to user
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.user_token_account.to_account_info(),
            mint: self.token_mint.to_account_info(),
            authority: self.vault_state.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer_checked(cpi_ctx, amount, self.token_mint.decimals)?;

        // Update state
        vault_state.total_deposited = vault_state
            .total_deposited
            .checked_sub(amount)
            .ok_or(CustomError::Overflow)?;
        user_account.deposited_amount = user_account
            .deposited_amount
            .checked_sub(amount)
            .ok_or(CustomError::Overflow)?;
        
            // Check if vault balance is zero after withdrawal
            if self.vault.amount == 0 {
                let accounts = CloseAccount {
                account: self.vault.to_account_info(),
                destination: self.user_token_account.to_account_info(),
                authority: self.vault_state.to_account_info(),
                };

                let ctx = CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                accounts,
                signer_seeds,
                );

                //close account
                close_account(ctx)?;
            }

        Ok(())
    }


    pub fn close_account(&mut self) -> Result<()> {

        let user_key = self.user.key();
        let seeds = &[
            b"vault_state".as_ref(),
            user_key.as_ref(),
            &self.vault_state.seed.as_bytes(),
            &[self.vault_state.state_bump],
        ];

        let signer_seeds = &[&seeds[..]];
        
        let accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.user_token_account.to_account_info(),
            authority: self.vault_state.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        //close account
        close_account(ctx)?;

        Ok(())
    }
}
