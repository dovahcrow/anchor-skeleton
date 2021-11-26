# Anchor Skeleton ![ci](https://img.shields.io/cirrus/github/dovahcrow/anchor-skeleton?style=flat-square&task=ci)

My project skeleton for [Anchor](https://github.com/project-serum/anchor) projects.

## Features
* Rust test only: All tests (integration, unit) are written in Rust so they will also be checked by the compiler. No Typescript based tests.
* CI integration: CI automatically deploys the project to devnet and run all the tests in Github Action.
* Feature based Program ID switch: switch program_id based on cargo features: by default the program_id will be the one decalre in `lib.rs` (production program-id).
  If the `dev` feature is enabled (enabled by default in the Justfile), the program_id will be loaded from `programs/contract-skeleton/key.json` (dev program-id).
  You can use `just keygen contract-skeleton` to generate a new one.
* Battery included: the skeleton demonstrates the usage of errors, events, examples and integration tests.
  
## Prerequisites

* System: Mac or Linux
* [Rust](https://rustup.rs/)
* [Just](https://github.com/casey/just#pre-built-binaries)
* [Solana](https://docs.solana.com/cli/install-solana-cli-tools#macos--linux)
* [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html#install-anchor)

For ci:
* Copy and paste your deployer key (a json array, e.g. the content of `~/.config/solana/id.json`) into the Github Secret `DEPLOY_KEY`.
  There should be some solana inside the deployer's account otherwise CI cannot deploy the project for paying the rent.
  
## Explanation on the recipies

* `just keygen contract-skeleton`: Generate a key.json into the program's folder. Once the `dev` feature is enabled, the program-id will be the one described in this key.
* `just deploy contract-skeleton`: Deploy the contract. Which network to deploy depends on your solana config (`solana config get`).
* `just test`: Run all the tests locally. `just test create_pool` to run the `create_pool` test specifically.
* `just cli`: Run the binaries defined in `programs/contract-skeleton/examples` folder. e.g. `just cli create_pool`. This uses the production program-id.
* `just devcli`: Run the binaries defined in `programs/contract-skeleton/examples` folder. e.g. `just cli create_pool`. This uses the dev program-id.
* `just idl contract-skeleton`: generate an idl to `target/idl`, and populate the program-id using the dev program-id.
* `just dangerously-close contract-skeleton`: Close the program and reclaim the rent.

## How-tos
* How do I add a new instruction? 
  First create a new context file in `programs/contract-skeleton/src/contexts.rs`.
  Then create the required account and implement the process method.
  Finally add a new method in the `lib.rs` that calls the process method. Try to make `lib.rs` clean so that one can easily know what's included in this program.
* How do I add a new error type? Add it in `programs/contract-skeleton/src/errors.rs`.
* How do I add a new event? Add it at the bottom of context file because an event usually only get emitted in one specific instruction.
* How do I add a new integration test? Add it in the tests folder. Take a look at the existing tests.