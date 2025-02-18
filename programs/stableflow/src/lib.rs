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

    pub fn config(ctx: Context<Config>, fee: u8) -> Result<()> {
    ctx.accounts.initialize_config(fee, &ctx.bumps)?;
    Ok(())
    }


    pub fn initialize(ctx: Context<Initialize>, init_pc_amount: u64, init_coin_amount: u64) -> Result<()> {
    ctx.accounts.initialize(init_pc_amount, init_coin_amount)?;
    Ok(())
    }


    pub fn deposit(ctx: Context<Stake>, max_coin_ammount: u64, max_pc_amount: u64) -> Result<()> {
    ctx.accounts.deposit(max_coin_ammount, max_pc_amount)?;
    Ok(())
    }
}
