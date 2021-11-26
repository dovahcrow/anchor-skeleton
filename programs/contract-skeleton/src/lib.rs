mod constraints;
pub mod contexts;
pub mod errors;
pub mod states;
mod utils;

pub use self::{
    constraints::suspended,
    contexts::*,
    errors::{ErrorCode, Result},
    states::*,
    utils::PDAIdentifier,
};
use anchor_lang::prelude::*;
use fehler::throws;
pub use program_id::*;
pub use program_id::*;

#[cfg(not(feature = "dev"))]
mod program_id {
    use anchor_lang::prelude::*;
    declare_id!("HeX9nsXz4GUrVcSAq6bdX8S8DsvryX4FQkfk4YqSZzRF");
}

#[cfg(feature = "dev")]
include!(concat!(env!("OUT_DIR"), "/program_id.rs"));

// Be sure to keep this module clean
// Write logics in each accounts' process method.
#[program]
pub mod skeleton {
    use super::*;

    // ======== User Instructions ========
    #[throws(ProgramError)]
    #[access_control(suspended(&ctx.accounts.pool))]
    pub fn user_do_nothing(ctx: Context<UserDoNothing>) {
        ctx.accounts.process()?;
    }

    // ======== Admin Instructions ========
    #[throws(ProgramError)]
    pub fn create_pool(ctx: Context<CreatePool>, seed: [u8; 32], bump: u8) {
        ctx.accounts.process(seed, bump)?;
    }
}
