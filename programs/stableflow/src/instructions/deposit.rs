use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use crate::{error::CustomError, ConfigState, UserState, VaultState};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub config: Account<'info, ConfigState>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub user_account: Account<'info, UserState>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = user,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub vault_state: Account<'info, VaultState>,

    #[account(mut)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let vault_state = &mut self.vault_state;
        let user = &mut self.user_account;
        let config = &mut self.config;

        // Transfer tokens from user to vault token account
        let cpi_accounts = TransferChecked {
            from: self.user_token_account.to_account_info(),
            to: self.vault.to_account_info(),
            mint: self.token_mint.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, amount, self.token_mint.decimals)?;

        // Update state
        config.total_deposits = config
            .total_deposits
            .checked_add(amount)
            .ok_or(CustomError::Overflow)?;
        vault_state.total_deposited = vault_state
            .total_deposited
            .checked_add(amount)
            .ok_or(CustomError::Overflow)?;
        user.deposited_amount = user
            .deposited_amount
            .checked_add(amount)
            .ok_or(CustomError::Overflow)?;

        emit!(DepositResposnse{
            amount,
            user_total: user.deposited_amount,
            vault_total: vault_state.total_deposited,
            user: self.user.key(),
            timestamp: Clock::get()?.unix_timestamp
        });

        Ok(())
    }
}


#[event]
pub struct DepositResposnse {
    pub user: Pubkey,
    pub amount: u64,
    pub vault_total: u64,
    pub user_total: u64,
    pub timestamp: i64,

    }