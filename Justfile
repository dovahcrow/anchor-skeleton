set dotenv-load := true

# ======== predefined recipes ========
default:
  @just --choose

build +ARGS="":
  anchor build {{ARGS}}

check:
  cargo check --tests --examples
  
cli prog +ARGS="":
  cargo run --example {{prog}} -- {{ARGS}}

devcli prog +ARGS="":
  cargo run --features dev --example {{prog}} -- {{ARGS}}

deploy which +ARGS="":
  anchor build -p $(echo "{{which}}" | sed -e "s/\-/_/g") -- --features dev {{ARGS}}
  solana program deploy --program-id programs/{{which}}/key.json target/deploy/$(echo "{{which}}" | sed -e "s/\-/_/g").so

test +ARGS="":
  cargo test {{ARGS}} --features dev -- --nocapture

idl which:
  #!/bin/bash
  mkdir -p target/deploy/idl
  anchor idl parse --file programs/{{which}}/src/lib.rs > target/idl/`echo "{{which}}"| sed -e 's/^.*\?-//g'`.json
  tmp=$(mktemp)
  jq --arg address $(solana-keygen pubkey programs/{{which}}/key.json) '. + {metadata: { address: $address }}' target/idl/$(echo "{{which}}"| sed -e 's/^.*\?-//g').json > "$tmp"
  mv "$tmp" target/idl/$(echo "{{which}}"| sed -e 's/^.*\?-//g').json

keygen which:
  solana-keygen new --no-passphrase -o programs/{{which}}/key.json

dangerously-close which:
  solana program close programs/{{which}}/key.json
  
# ======== Begin custom commands ========
