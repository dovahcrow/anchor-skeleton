mod utils;

use anchor_client::Client;
use anyhow::Error;
use fehler::throws;
use gfx_solana_utils::{admin_wallet, cluster, commitment_level, user_wallet, Duplicate};

#[throws(Error)]
#[test]
fn user_do_nothing() {
    let _ = env_logger::try_init();
    println!("program_id: {}", contract_skeleton::ID);

    let admin = admin_wallet(1.)?;
    let user_wallet = user_wallet(1.)?;

    let client = Client::new_with_options(cluster(), admin.clone(), commitment_level());
    let program = client.program(contract_skeleton::ID);

    let pool = utils::create_pool(&program, admin)?;
    utils::user_do_nothing(&program, pool, user_wallet)?;
}
