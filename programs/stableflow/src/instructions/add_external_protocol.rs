use crate::ExternalProtocolState;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(pool_id: Pubkey, name: String)]
pub struct AddExternalProtocol<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init, 
        payer = user, 
        seeds = [b"external_protocol", pool_id.to_bytes().as_ref(), name.as_bytes()],
        space = 8 + ExternalProtocolState::INIT_SPACE,
        bump
    )]
    pub protocol: Account<'info, ExternalProtocolState>,

    pub system_program: Program<'info, System>,
}

impl<'info> AddExternalProtocol<'info> {
    pub fn add(
        &mut self,
        pool_id: Pubkey,
        name: String,
        bumps: &AddExternalProtocolBumps,
    ) -> Result<()> {
        self.protocol.set_inner(ExternalProtocolState {
            pool_id,
            allocated_funds: 0,
            name,
            bump: bumps.protocol,
        });
        Ok(())
    }
}
