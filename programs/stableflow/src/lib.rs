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

    pub fn add_external_protocol(ctx: Context<AddExternalProtocol>, protocol_id: String) -> Result<()> {
        ctx.accounts.add(protocol_id)?;
        Ok(())
    }
}
