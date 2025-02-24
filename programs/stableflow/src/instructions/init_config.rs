use anchor_lang::prelude::*;

use crate::ConfigState;


#[derive(Accounts)]
pub struct Config<'info>  {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init, 
        payer = admin,
        seeds = [b"config"],
        space = 8 + ConfigState::INIT_SPACE,
        bump,
    )]
    pub config : Account<'info, ConfigState>,
    pub system_program: Program<'info, System>,
}

impl<'info> Config<'info> {
    pub fn initialize_config(&mut self, fee: u16, bumps: &ConfigBumps) -> Result<()> {
        self.config.set_inner(ConfigState{
            admin: self.admin.key(),
            fee,
            total_deposits: 0,
            total_yield: 0,
            bump: bumps.config,
        });
        Ok(())
    }
}