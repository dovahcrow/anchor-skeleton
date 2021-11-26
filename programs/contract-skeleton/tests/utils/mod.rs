#![allow(dead_code)]

use anchor_client::Program;
use anchor_lang::prelude::*;
use anyhow::Error;
use contract_skeleton::{ErrorCode, PDAIdentifier, Pool};
use fehler::throws;
use gfx_solana_utils::AnchorClientErrorExt;
use solana_sdk::{signature::Keypair, signature::Signer, system_program, sysvar};

#[throws(Error)]
pub fn create_pool(program: &Program, admin: &Keypair) -> Pubkey {
    let seed = Keypair::new().pubkey().to_bytes();

    let (pool, bump) = Pool::get_address_with_bump(&contract_skeleton::ID, &[&seed]);

    let tx = program
        .request()
        .accounts(contract_skeleton::accounts::CreatePool {
            pool,
            admin: admin.pubkey(),
            system_program: system_program::id(),
            rent: sysvar::rent::id(),
        })
        .args(contract_skeleton::instruction::CreatePool { seed, bump })
        .signer(admin)
        .send()
        .map_err(|e| e.canonicalize::<ErrorCode>())?;

    println!(
        "CreatePool: https://explorer.solana.com/tx/{}?cluster=devnet",
        tx
    );

    pool
}

#[throws(Error)]
pub fn user_do_nothing(program: &Program, pool: Pubkey, user_wallet: &Keypair) {
    let tx = program
        .request()
        .accounts(contract_skeleton::accounts::UserDoNothing {
            pool,
            user_wallet: user_wallet.pubkey(),
        })
        .args(contract_skeleton::instruction::UserDoNothing {})
        .signer(user_wallet)
        .send()
        .map_err(|e| e.canonicalize::<ErrorCode>())?;

    println!(
        "UserDoNothing: https://explorer.solana.com/tx/{}?cluster=devnet",
        tx
    );
}
