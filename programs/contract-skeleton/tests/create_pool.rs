mod utils;

use anchor_client::Client;
use anyhow::Error;
use fehler::throws;
use gfx_solana_utils::{admin_wallet, cluster, commitment_level, Duplicate};

#[throws(Error)]
#[test]
fn create_pool() {
    let _ = env_logger::try_init();
    println!("program_id: {}", contract_skeleton::ID);

    let admin = admin_wallet(1.)?;

    let client = Client::new_with_options(cluster(), admin.clone(), commitment_level());
    let program = client.program(contract_skeleton::ID);

    utils::create_pool(&program, admin)?;
}
