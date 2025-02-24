use std::u64;

use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::Token};
use raydium_amm_cpi::{
    create_pool_fee_address, initialize, openbook_program_id, Initialize2,
};

#[derive(Accounts)]
pub struct RaydiumAmmInitialize<'info> {
    /// CHECK: Safe
    pub amm_program: UncheckedAccount<'info>,

    /// CHECK: Safe. The new amm Account to be create, a PDA create with seed = [program_id, openbook_market_id, b"amm_associated_seed"]
    #[account(
        mut,
        seeds = [
            crate::id().as_ref(),
            market.key.as_ref(),
            b"amm_associated_seed",
        ],
        bump,
    )]
    pub amm: UncheckedAccount<'info>,
    /// CHECK: Safe. Amm authority, a PDA create with seed = [b"amm authority"]
    #[account(
        seeds = [b"amm authority"],
        bump,
    )]
    pub amm_authority: UncheckedAccount<'info>,
    /// CHECK: Safe. Amm open_orders Account, a PDA create with seed = [program_id, openbook_market_id, b"open_order_associated_seed"]
    #[account(
        mut,
        seeds = [
            crate::id().as_ref(),
            market.key.as_ref(),
            b"open_order_associated_seed",
        ],
        bump,
    )]
    pub amm_open_orders: UncheckedAccount<'info>,
    /// CHECK: Safe. Pool lp mint account. Must be empty, owned by $authority.
    #[account(
        mut,
        seeds = [
            crate::id().as_ref(),
            market.key.as_ref(),
            b"lp_mint_associated_seed",
        ],
        bump,
    )]
    pub amm_lp_mint: UncheckedAccount<'info>,
    /// CHECK: Safe. Coin mint account
    #[account(
        owner = token_program.key()
    )]
    pub amm_coin_mint: UncheckedAccount<'info>,
    /// CHECK: Safe. Pc mint account
    #[account(
        owner = token_program.key()
    )]
    pub amm_pc_mint: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_coin_vault Account. Must be non zero, owned by $authority
    #[account(
        mut,
        seeds = [
            crate::id().as_ref(),
            market.key.as_ref(),
            b"coin_vault_associated_seed",
        ],
        bump,
    )]
    pub amm_coin_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_pc_vault Account. Must be non zero, owned by $authority.
    #[account(
        mut,
        seeds = [
            crate::id().as_ref(),
            market.key.as_ref(),
            b"pc_vault_associated_seed",
        ],
        bump,
    )]
    pub amm_pc_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_target_orders Account. Must be non zero, owned by $authority.
    #[account(
        mut,
        seeds = [
            crate::id().as_ref(),
            market.key.as_ref(),
            b"target_associated_seed",
        ],
        bump,
    )]
    pub amm_target_orders: UncheckedAccount<'info>,
    /// CHECK: Safe. Amm Config.
    #[account(
        seeds = [b"amm_config_account_seed"],
        bump,
    )]
    pub amm_config: UncheckedAccount<'info>,
    /// CHECK: Safe. Amm create_fee_destination.
    #[account(
        mut,
        address = create_pool_fee_address::id(),
    )]
    pub create_fee_destination: UncheckedAccount<'info>,

    /// CHECK: Safe. OpenBook program.
    #[account(
        address = openbook_program_id::id(),
    )]
    pub market_program: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook market. OpenBook program is the owner.
    #[account(
        owner = market_program.key(),
    )]
    pub market: UncheckedAccount<'info>,
    /// CHECK: Safe. The user wallet create the pool
    #[account(mut)]
    pub user_wallet: Signer<'info>,
    /// CHECK: Safe. The user coin token
    #[account(
        mut,
        owner = token_program.key(),
    )]
    pub user_token_coin: UncheckedAccount<'info>,
    /// CHECK: Safe. The user pc token
    #[account(
        mut,
        owner = token_program.key(),
    )]
    pub user_token_pc: UncheckedAccount<'info>,
    /// CHECK: Safe. The user lp token
    #[account(
        mut,
        seeds = [
            &user_wallet.key().to_bytes(),
            &token_program.key().to_bytes(),
            &amm_lp_mint.key.to_bytes(),
            ],
        bump,
    )]
    pub user_token_lp: UncheckedAccount<'info>,
    /// CHECK: Safe. The spl token program
    pub token_program: Program<'info, Token>,
    /// CHECK: Safe. The associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: Safe. System program
    pub system_program: Program<'info, System>,
    /// CHECK: Safe. Rent program
    pub sysvar_rent: Sysvar<'info, Rent>,
}

impl<'info> RaydiumAmmInitialize<'info> {
    pub fn initialize(&mut self,nonce: u8, open_time: u64, init_pc_amount: u64, init_coin_amount: u64) -> Result<()> {
        let cpi_accounts = Initialize2 {
            amm: self.amm.clone(),
            amm_authority: self.amm_authority.clone(),
            amm_coin_mint: self.amm_coin_mint.clone(),
            amm_coin_vault: self.amm_coin_vault.clone(),
            amm_config: self.amm_config.clone(),
            amm_lp_mint: self.amm_lp_mint.clone(),
            amm_open_orders: self.amm_open_orders.clone(),
            amm_pc_mint: self.amm_pc_mint.clone(),
            amm_pc_vault: self.amm_pc_vault.clone(),
            amm_target_orders: self.amm_target_orders.clone(),
            create_fee_destination: self.create_fee_destination.clone(),
            market: self.market.clone(),
            user_token_coin: self.user_token_coin.clone(),
            user_token_lp: self.user_token_lp.clone(),
            user_token_pc: self.user_token_pc.clone(),
            user_wallet: self.user_wallet.clone(),
            market_program: self.market_program.clone(),
            associated_token_program: self.associated_token_program.clone(),
            system_program: self.system_program.clone(),
            token_program: self.token_program.clone(),
            sysvar_rent: self.sysvar_rent.clone(),
        };

        let ctx = CpiContext::new(self.amm_program.to_account_info(), cpi_accounts);
        initialize(ctx, nonce, open_time, init_pc_amount, init_coin_amount)?;

        Ok(())
    }
}
