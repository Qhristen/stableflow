use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use raydium_amm_cpi::{withdraw, Withdraw};

#[derive(Accounts)]
pub struct RaydiumAmmWithdraw<'info> {
    /// CHECK: Safe
    pub amm_program: UncheckedAccount<'info>,

    /// CHECK: Safe. Amm account
    #[account(mut)]
    pub amm: UncheckedAccount<'info>,
    /// CHECK: Safe. Amm authority Account
    #[account(
         seeds = [b"amm authority"],
         bump,
     )]
    pub amm_authority: UncheckedAccount<'info>,
    /// CHECK: Safe. amm open_orders Account
    #[account(mut)]
    pub amm_open_orders: UncheckedAccount<'info>,
    /// CHECK: Safe. amm target_orders Account. To store plan orders infomations.
    #[account(mut)]
    pub amm_target_orders: UncheckedAccount<'info>,
    /// CHECK: Safe. pool lp mint account. Must be empty, owned by $authority.
    #[account(mut)]
    pub amm_lp_mint: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_coin_vault Amm Account to withdraw FROM,
    #[account(mut)]
    pub amm_coin_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_pc_vault Amm Account to withdraw FROM,
    #[account(mut)]
    pub amm_pc_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook program id
    pub market_program: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook market Account. OpenBook program is the owner.
    #[account(mut)]
    pub market: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook coin_vault Account
    #[account(mut)]
    pub market_coin_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook pc_vault Account
    #[account(mut)]
    pub market_pc_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook vault_signer Account
    pub market_vault_signer: UncheckedAccount<'info>,
    /// CHECK: Safe. user lp token Account. Source lp, amount is transferable by $authority.
    #[account(mut)]
    pub user_token_lp: UncheckedAccount<'info>,
    /// CHECK: Safe. user token coin Account. user Account to credit.
    #[account(mut)]
    pub user_token_coin: UncheckedAccount<'info>,
    /// CHECK: Safe. user token pc Account. user Account to credit.
    #[account(mut)]
    pub user_token_pc: UncheckedAccount<'info>,
    /// CHECK: Safe. User wallet account
    #[account(mut)]
    pub user_owner: Signer<'info>,
    /// CHECK: Safe. OpenBook event queue account
    #[account(mut)]
    pub market_event_q: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook bid account
    #[account(mut)]
    pub market_bids: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook ask account
    #[account(mut)]
    pub market_asks: UncheckedAccount<'info>,
    /// CHECK: Safe. The spl token program
    pub token_program: Program<'info, Token>,
}

impl<'info> RaydiumAmmWithdraw<'info> {
    pub fn withdraw_reward(&mut self, amount: u64) -> Result<()> {

        let cpi_accounts = Withdraw{
            amm: self.amm.clone(),
            amm_authority: self.amm_authority.clone(),
            amm_coin_vault: self.amm_coin_vault.clone(),
            amm_lp_mint: self.amm_lp_mint.clone(),
            amm_open_orders: self.amm_open_orders.clone(),
            amm_pc_vault: self.amm_pc_vault.clone(),
            amm_target_orders: self.amm_target_orders.clone(),
            market: self.market.clone(),
            market_asks: self.market_asks.clone(),
            market_bids: self.market_bids.clone(),
            market_coin_vault: self.market_coin_vault.clone(),
            market_event_q: self.market_event_q.clone(),
            market_pc_vault: self.market_pc_vault.clone(),
            market_program: self.market_program.clone(),
            market_vault_signer: self.market_vault_signer.clone(),
            token_program: self.token_program.clone(),
            user_owner: self.user_owner.clone(),
            user_token_coin: self.user_token_coin.clone(),
            user_token_lp: self.user_token_lp.clone(),
            user_token_pc: self.user_token_pc.clone()
        };

        let cpi_context = CpiContext::new(self.amm_program.to_account_info(), cpi_accounts);
        withdraw(cpi_context, amount)?;
        
        Ok(())
    }
}
