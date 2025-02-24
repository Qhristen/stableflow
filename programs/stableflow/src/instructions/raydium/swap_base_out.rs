use anchor_lang::prelude::*;
use anchor_spl::token::Token;

#[derive(Accounts)]
pub struct RaydiumAmmSwapBaseOut<'info> {
    /// CHECK: Safe
    pub amm_program: UncheckedAccount<'info>,

    /// CHECK: Safe. amm Account
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
    /// CHECK: Safe. amm_coin_vault Amm Account to swap FROM or To,
    #[account(mut)]
    pub amm_coin_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_pc_vault Amm Account to swap FROM or To,
    #[account(mut)]
    pub amm_pc_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook program id
    pub market_program: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook market Account. OpenBook program is the owner.
    #[account(mut)]
    pub market: UncheckedAccount<'info>,
    /// CHECK: Safe. bids Account
    #[account(mut)]
    pub market_bids: UncheckedAccount<'info>,
    /// CHECK: Safe. asks Account
    #[account(mut)]
    pub market_asks: UncheckedAccount<'info>,
    /// CHECK: Safe. event_q Account
    #[account(mut)]
    pub market_event_queue: UncheckedAccount<'info>,
    /// CHECK: Safe. coin_vault Account
    #[account(mut)]
    pub market_coin_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. pc_vault Account
    #[account(mut)]
    pub market_pc_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. vault_signer Account
    #[account(mut)]
    pub market_vault_signer: UncheckedAccount<'info>,
    /// CHECK: Safe. user source token Account. user Account to swap from.
    #[account(mut)]
    pub user_token_source: UncheckedAccount<'info>,
    /// CHECK: Safe. user destination token Account. user Account to swap to.
    #[account(mut)]
    pub user_token_destination: UncheckedAccount<'info>,
    /// CHECK: Safe. user owner Account
    #[account(mut)]
    pub user_source_owner: Signer<'info>,
    /// CHECK: Safe. The spl token program
    pub token_program: Program<'info, Token>,
}

impl<'info> RaydiumAmmSwapBaseOut<'info> {
    pub fn swap_base_out(&mut self, max_amount_in: u64, amount_out: u64) -> Result<()> {
        let cpi_accounts = raydium_amm_cpi::SwapBaseOut {
            amm: self.amm.clone(),
            amm_authority: self.amm_authority.clone(),
            amm_open_orders: self.amm_open_orders.clone(),
            amm_coin_vault: self.amm_coin_vault.clone(),
            amm_pc_vault: self.amm_pc_vault.clone(),
            market_program: self.market_program.clone(),
            market: self.market.clone(),
            market_bids: self.market_bids.clone(),
            market_asks: self.market_asks.clone(),
            market_event_queue: self.market_event_queue.clone(),
            market_coin_vault: self.market_coin_vault.clone(),
            market_pc_vault: self.market_pc_vault.clone(),
            market_vault_signer: self.market_vault_signer.clone(),
            user_token_source: self.user_token_source.clone(),
            user_token_destination: self.user_token_destination.clone(),
            user_source_owner: self.user_source_owner.clone(),
            token_program: self.token_program.clone(),
        };

        let cpi_context = CpiContext::new(self.amm_program.to_account_info(), cpi_accounts);
        raydium_amm_cpi::swap_base_out(cpi_context, max_amount_in, amount_out)?;

        Ok(())
    }
}
