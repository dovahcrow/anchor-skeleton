# Anchor Skeleton ![ci](https://img.shields.io/github/workflow/status/dovahcrow/anchor-skeleton/ci?style=flat-square)

Battery-included Solana/[Anchor](https://github.com/project-serum/anchor) project skeleton.

## Features
* Rust test only: All tests (integration, unit) are written in Rust, so the compiler will also check them. No Typescript based tests.
* CI integration: CI automatically deploys the project to Devnet and runs all the tests in Github Action.
* Feature-based Program ID switch: switch the `declare_id` in `lib.rs` based on cargo features: by default, the program_id will be declared in `lib.rs` (production program-id).
  If the `dev` cargo feature is enabled (enabled by default in the Justfile), the `declare_id` will be the one loaded from `programs/contract-skeleton/key.json` (dev program-id).
  You can use `just keygen contract-skeleton` to generate a new one.
* Battery included: the skeleton demonstrates the usage of errors, events, examples and integration tests and is immediately deployable and testable.
  
## Prerequisites

* System: Mac or Linux
* [Rust](https://rustup.rs/)
* [Just](https://github.com/casey/just#pre-built-binaries)
* [Solana](https://docs.solana.com/cli/install-solana-cli-tools#macos--linux)
* [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html#install-anchor)

For CI:
* Copy and paste your deployer key (a JSON array, e.g. the content of `~/.config/solana/id.json`) into the Github Secret `DEPLOY_KEY`.
  There should be some SOL inside the deployer's account; otherwise, CI cannot deploy the project to pay the rent.
  
## Explanation on the Justfile recipes

* `just keygen contract-skeleton`: Generate a key.json into the program's folder. Once the `dev` feature is enabled, the program-id will be the one described in this key.
* `just deploy contract-skeleton`: Deploy the contract. Which network to deploy depends on your Solana config (`solana config get`).
* `just test`: Run all the tests locally. `just test create_pool` to run the `create_pool` test specifically.
* `just cli`: Run the binaries defined in `programs/contract-skeleton/examples` folder. e.g. `just cli create_pool`. This command uses the production program-id.
* `just devcli`: Run the binaries defined in `programs/contract-skeleton/examples` folder. e.g. `just cli create_pool`. This uses the dev program-id.
* `just idl contract-skeleton`: generate an idl to `target/idl`, and populate the program-id using the dev program-id.
* `just dangerously-close contract-skeleton`: Close the program and reclaim the rent.

## How-tos
* How do I add a new instruction? 
  First, create a new context file in `programs/contract-skeleton/src/contexts.rs`.
  Then create the required account and implement the process method.
  Finally, add a new method in the `lib.rs` that calls the process method. Try to make `lib.rs` clean for someone to know what's included in this program quickly.
* How do I add a new error type? Add it in `programs/contract-skeleton/src/errors.rs`.
* How do I add a new event? Add it at the bottom of the context file because an event usually only gets emitted in one specific instruction.
* How do I add a new integration test? Add it to the tests folder. Also take a look at the existing tests.
