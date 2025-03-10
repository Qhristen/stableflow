use anchor_lang::prelude::*;
use anchor_spl::{
    token::Token,
    token_2022::Token2022,
    token_interface::{Mint, TokenAccount},
};
use raydium_cpmm_cpi::{cpi, program::RaydiumCpmm, states::PoolState};

#[derive(Accounts)]
pub struct CpmmDeposit<'info> {
    /// cpmm program
    pub cp_swap_program: Program<'info, RaydiumCpmm>,

    /// Pays to mint the position
    pub owner: Signer<'info>,

    /// CHECK: pool vault and lp mint authority
    #[account(
        seeds = [
            raydium_cpmm_cpi::AUTH_SEED.as_bytes(),
        ],
        seeds::program = cp_swap_program.key(),
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// Owner lp tokan account
    #[account(mut,  token::authority = owner)]
    pub owner_lp_token: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The payer's token account for token_0
    #[account(
        mut,
        token::mint = token_0_vault.mint,
        token::authority = owner
    )]
    pub token_0_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The payer's token account for token_1
    #[account(
        mut,
        token::mint = token_1_vault.mint,
        token::authority = owner
    )]
    pub token_1_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_0
    #[account(
        mut,
        constraint = token_0_vault.key() == pool_state.load()?.token_0_vault
    )]
    pub token_0_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_1
    #[account(
        mut,
        constraint = token_1_vault.key() == pool_state.load()?.token_1_vault
    )]
    pub token_1_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// token Program
    pub token_program: Program<'info, Token>,

    /// Token program 2022
    pub token_program_2022: Program<'info, Token2022>,

    /// The mint of token_0 vault
    #[account(
        address = token_0_vault.mint
    )]
    pub vault_0_mint: Box<InterfaceAccount<'info, Mint>>,

    /// The mint of token_1 vault
    #[account(
        address = token_1_vault.mint
    )]
    pub vault_1_mint: Box<InterfaceAccount<'info, Mint>>,

    /// Lp token mint
    #[account(
        mut,
        address = pool_state.load()?.lp_mint)
    ]
    pub lp_mint: Box<InterfaceAccount<'info, Mint>>,
}

impl<'info> CpmmDeposit<'info> {
    pub fn proxy_deposit(
        &mut self,
        lp_token_amount: u64,
        maximum_token_0_amount: u64,
        maximum_token_1_amount: u64,
    ) -> Result<()> {
        let cpi_accounts = cpi::accounts::Deposit {
            owner: self.owner.to_account_info(),
            authority: self.authority.to_account_info(),
            pool_state: self.pool_state.to_account_info(),
            owner_lp_token: self.owner_lp_token.to_account_info(),
            token_0_account: self.token_0_account.to_account_info(),
            token_1_account: self.token_1_account.to_account_info(),
            token_0_vault: self.token_0_vault.to_account_info(),
            token_1_vault: self.token_1_vault.to_account_info(),
            token_program: self.token_program.to_account_info(),
            token_program_2022: self.token_program_2022.to_account_info(),
            vault_0_mint: self.vault_0_mint.to_account_info(),
            vault_1_mint: self.vault_1_mint.to_account_info(),
            lp_mint: self.lp_mint.to_account_info(),
        };
        let cpi_context = CpiContext::new(self.cp_swap_program.to_account_info(), cpi_accounts);
        cpi::deposit(
            cpi_context,
            lp_token_amount,
            maximum_token_0_amount,
            maximum_token_1_amount,
        )
    }

    }