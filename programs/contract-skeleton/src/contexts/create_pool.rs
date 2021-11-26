use crate::states::Pool;
use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;
use fehler::throws;

#[derive(Accounts)]
#[instruction(seed: [u8; 32], bump: u8)]
pub struct CreatePool<'info> {
    #[account(
        init,
        seeds = [Pool::IDENT, &seed],
        bump = bump,
        payer = admin
    )]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> CreatePool<'info> {
    #[throws(ProgramError)]
    pub fn process(&mut self, seed: [u8; 32], bump: u8) {
        let CreatePool { pool, admin, .. } = self;

        pool.initialize(&seed, bump, admin.key());
    }
}
