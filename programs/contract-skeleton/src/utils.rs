use crate::errors::ErrorCode;
use anchor_lang::prelude::*;
use fehler::{throw, throws};

// All the PDA account of this program are derived from a same seed with `find_program_address(IDENT, seed)`.
pub trait PDAIdentifier {
    const IDENT: &'static [u8];

    fn get_address(program_id: &Pubkey, seeds: &[&[u8]]) -> Pubkey {
        Self::get_address_with_bump(program_id, seeds).0
    }

    fn get_address_with_bump(program_id: &Pubkey, seeds: &[&[u8]]) -> (Pubkey, u8) {
        // TODO: avoid heap allocation
        let mut seeds = seeds.to_vec();
        seeds.insert(0, Self::IDENT);
        Pubkey::find_program_address(&seeds, program_id)
    }

    #[throws(ProgramError)]
    fn verify_address(program_id: &Pubkey, seeds: &[&[u8]], address: &Pubkey) {
        let (expected, _) = Self::get_address_with_bump(program_id, seeds);

        if &expected != address {
            throw!(ErrorCode::ContractAddressNotCorrect);
        }
    }

    #[throws(ProgramError)]
    fn verify_address_with_bump(program_id: &Pubkey, seeds: &[&[u8]], bump: u8, address: &Pubkey) {
        // TODO: avoid heap allocation
        let mut seeds = seeds.to_vec();
        seeds.insert(0, Self::IDENT);
        let bump = &[bump];

        seeds.push(bump);

        let addr = Pubkey::create_program_address(&seeds, program_id)?;

        if &addr != address {
            throw!(ErrorCode::ContractAddressNotCorrect);
        }
    }
}
