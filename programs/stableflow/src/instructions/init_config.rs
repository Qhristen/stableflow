use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use crate::ConfigState;


#[derive(Accounts)]
pub struct Config<'info>  {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init, 
        payer = admin,
        seeds = [b"intitialize_config".as_ref()],
        space = 8 + ConfigState::INIT_SPACE,
        bump,
    )]
    pub config : Account<'info, ConfigState>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Config<'info> {
    pub fn initialize_config(&mut self, fee: u8, bumps: &ConfigBumps) -> Result<()> {
        self.config.set_inner(ConfigState{
            admin: self.admin.key(),
            bump: bumps.config,
           treasury_fee: fee,
        });
        Ok(())
    }
}