use crate::states::Pool;
use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;
use fehler::throws;

#[derive(Accounts)]
pub struct UserDoNothing<'info> {
    #[account(
        seeds = [Pool::IDENT, &pool.seed],
        bump = pool.bump,
    )]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub user_wallet: Signer<'info>,
}

impl<'info> UserDoNothing<'info> {
    #[throws(ProgramError)]
    pub fn process(&mut self) {
        let UserDoNothing { user_wallet, .. } = self;

        emit!(UserDidNothing {
            who: user_wallet.key()
        })
    }
}

#[event]
pub struct UserDidNothing {
    pub who: Pubkey,
}
