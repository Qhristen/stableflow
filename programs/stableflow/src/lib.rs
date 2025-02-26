pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("DUbyG5Yoyrd26hV28BUH5A5Wmr76jzQsmxuaMmv1fdJn");

#[program]
pub mod stableflow {
    use super::*;

    pub fn config(ctx: Context<Config>, fee: u16) -> Result<()> {
        ctx.accounts.initialize_config(fee, &ctx.bumps)?;
        Ok(())
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.init_user(&ctx.bumps)?;
        Ok(())
    }

    pub fn initialize_vault(ctx: Context<InitializeVault>, seed: String) -> Result<()> {
        ctx.accounts.init_vault(seed, &ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        ctx.accounts.claim()?;
        Ok(())
    }

    pub fn add_external_protocol(
        ctx: Context<AddExternalProtocol>,
        pool_id: Pubkey,
        name: String,
    ) -> Result<()> {
        ctx.accounts.add(pool_id, name, &ctx.bumps)?;
        Ok(())
    }

    pub fn create_cpmm_pool(
        ctx: Context<CreateCpmmPool>,
        funding_amount: Option<u64>,
    ) -> Result<()> {
        ctx.accounts.issue_tokens()?;
        ctx.accounts.revoke_mint_authority()?;
        ctx.accounts.create_cpmm_pool(funding_amount)
    }

    pub fn lock_cpmm_liquidity(ctx: Context<LockCpmmLiquidity>) -> Result<()> {
        ctx.accounts.lock_cpmm_cpi()
    }

    pub fn cpmm_deposit(
        ctx: Context<CpmmDeposit>,
        lp_token_amount: u64,
        maximum_token_0_amount: u64,
        maximum_token_1_amount: u64,
    ) -> Result<()> {
        ctx.accounts.proxy_deposit(
            lp_token_amount,
            maximum_token_0_amount,
            maximum_token_1_amount,
        )
    }
}
