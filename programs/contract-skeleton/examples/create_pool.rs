use anchor_client::Client;
use anyhow::Error;
use contract_skeleton::{ErrorCode, PDAIdentifier, Pool};
use fehler::throws;
use gfx_solana_utils::{admin_wallet, cluster, commitment_level, AnchorClientErrorExt, Duplicate};
use solana_sdk::{signature::Keypair, signature::Signer, system_program, sysvar};

#[throws(Error)]
fn main() {
    let _ = env_logger::try_init();

    println!("program_id: {}", contract_skeleton::ID);

    let admin = admin_wallet(1.)?;

    let client = Client::new_with_options(cluster(), admin.clone(), commitment_level());
    let program = client.program(contract_skeleton::ID);

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
}
