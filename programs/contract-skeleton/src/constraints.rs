use crate::states::Pool;
use anchor_lang::prelude::*;
use fehler::throws;

#[throws(ProgramError)]
pub fn suspended(pool: &Pool) {
    require!(!pool.suspended, Suspended);
}
