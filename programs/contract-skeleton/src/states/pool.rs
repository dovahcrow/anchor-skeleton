use crate::utils::PDAIdentifier;
use anchor_lang::prelude::*;

impl PDAIdentifier for Pool {
    const IDENT: &'static [u8] = b"POOL";
}

#[account]
#[derive(Default)]
pub struct Pool {
    pub seed: [u8; 32],
    pub bump: u8,
    pub admin: Pubkey,
    pub suspended: bool,
}

impl Pool {
    pub fn initialize(&mut self, seed: &[u8; 32], bump: u8, admin: Pubkey) {
        self.seed = *seed;
        self.bump = bump;
        self.admin = admin;
    }
}
