use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use raydium_amm_cpi::{deposit, Deposit};

#[derive(Accounts)]
pub struct RaydiumAmmDeposit<'info> {
    /// CHECK: Safe
    pub amm_program: UncheckedAccount<'info>,

    /// CHECK: Safe. Amm Account
    #[account(mut)]
    pub amm: UncheckedAccount<'info>,
    /// CHECK: Safe. Amm authority, a PDA create with seed = [b"amm authority"]
    #[account(
       seeds = [b"amm authority"],
       bump,
   )]
    pub amm_authority: UncheckedAccount<'info>,
    /// CHECK: Safe. AMM open_orders Account.
    #[account()]
    pub amm_open_orders: UncheckedAccount<'info>,
    /// CHECK: Safe. AMM target orders account. To store plan orders infomations.
    #[account(mut)]
    pub amm_target_orders: UncheckedAccount<'info>,
    /// CHECK: Safe. LP mint account. Must be empty, owned by $authority.
    #[account(mut)]
    pub amm_lp_mint: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_coin_vault account, $authority can transfer amount.
    #[account(mut)]
    pub amm_coin_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_pc_vault account, $authority can transfer amount.
    #[account(mut)]
    pub amm_pc_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook market account, OpenBook program is the owner.
    pub market: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook market event queue account, OpenBook program is the owner.
    pub market_event_queue: UncheckedAccount<'info>,
    /// CHECK: Safe. User token coin to deposit into.
    #[account(mut)]
    pub user_token_coin: UncheckedAccount<'info>,
    /// CHECK: Safe. User token pc to deposit into.
    #[account(mut)]
    pub user_token_pc: UncheckedAccount<'info>,
    /// CHECK: Safe. User lp token, to deposit the generated tokens, user is the owner
    #[account(mut)]
    pub user_token_lp: UncheckedAccount<'info>,
    /// CHECK: Safe. User wallet account
    #[account(mut)]
    pub user_owner: Signer<'info>,
    /// CHECK: Safe. The spl token program
    pub token_program: Program<'info, Token>,
}

impl<'info> RaydiumAmmDeposit<'info> {
    pub fn deposit(&mut self, max_coin_amount: u64, max_pc_amount: u64, base_side: u64) -> Result<()> {
        let cpi_accounts = Deposit {
            amm: self.amm.clone(),
            amm_authority: self.amm_authority.clone(),
            amm_coin_vault: self.amm_coin_vault.clone(),
            amm_lp_mint: self.amm_lp_mint.clone(),
            amm_open_orders: self.amm_open_orders.clone(),
            amm_pc_vault: self.amm_pc_vault.clone(),
            amm_target_orders: self.amm_target_orders.clone(),
            market: self.market.clone(),
            market_event_queue: self.market_event_queue.clone(),
            token_program: self.token_program.clone(),
            user_owner: self.user_owner.clone(),
            user_token_coin: self.user_token_coin.clone(),
            user_token_lp: self.user_token_lp.clone(),
            user_token_pc: self.user_token_pc.clone(),
        };

        let ctx = CpiContext::new(self.amm_program.to_account_info(), cpi_accounts);
        deposit(ctx, max_coin_amount, max_pc_amount, base_side)?;

        Ok(())
    }
}
