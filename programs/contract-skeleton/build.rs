#[cfg(feature = "dev")]
fn main() {
    use gfx_solana_utils::load_keypair;
    use solana_sdk::signer::Signer;
    use std::env;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::path::Path;

    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=key.json");

    if !Path::new("./key.json").exists() {
        panic!("key.json does not exist");
    }

    let keypair = load_keypair("./key.json").unwrap();

    let pubkey = keypair.pubkey();

    let content = format!(
        r#"mod program_id {{ use anchor_lang::prelude::*; declare_id!("{}"); }}"#,
        pubkey
    );

    let out_dir = env::var("OUT_DIR").unwrap();
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&format!("{}/program_id.rs", out_dir))
        .unwrap();

    f.write(content.as_bytes()).unwrap();
}

#[cfg(not(feature = "dev"))]
fn main() {}
