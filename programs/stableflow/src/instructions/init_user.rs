use anchor_lang::prelude::*;

use crate::UserState;

#[derive(Accounts)]
pub struct InitializeUser<'info>  {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init, 
        payer = user, 
        seeds = [b"user", user.key().as_ref()],
        space = 8 + UserState::INIT_SPACE,
        bump
    )]
    pub user_account: Account<'info, UserState>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeUser<'info> {
    pub fn init_user(&mut self, bumps: &InitializeUserBumps ) -> Result<()> {
        self.user_account.set_inner(UserState{
            deposited_amount: 0,
            pending_rewards: 0,
            bump: bumps.user_account,
            user: self.user_account.key(),
            vault: self.user_account.vault
        });
        
        Ok(())

    
    }
}