# Trappist

[![Check Set-Up & Build](https://github.com/paritytech/trappist/actions/workflows/check.yml/badge.svg)](https://github.com/paritytech/trappist/actions/workflows/check.yml)
[![XCM Simulator](https://github.com/paritytech/trappist/actions/workflows/simulate.yml/badge.svg)](https://github.com/paritytech/trappist/actions/workflows/simulate.yml)

**Trappist** is a web3 developer playground for experimenting with [cross-chain applications and services](https://polkadot.network/cross-chain-communication/) built on the technologies spearheaded by the [Polkadot Network](https://polkadot.network/), namely:
* [Substrate](https://substrate.io/), a Blockchain framework that enables developers to quickly and easily build future proof blockchains optimized for any use case.
* [Cumulus](https://github.com/paritytech/cumulus), a set of tools for writing Substrate-based Polkadot parachains.
* [XCM](https://polkadot.network/cross-chain-communication/), a common language for secure messaging across Polkadot  parachains, and with external networks via bridges.
* [Rococo](https://polkadot.network/blog/statemint-becomes-first-common-good-parachain-on-polkadot/), Polkadot’s Parachain Testnet.
* [Statemint](https://polkadot.network/blog/statemint-becomes-first-common-good-parachain-on-polkadot/), Polkadot's common good parachain which provides functionality for deploying and transferring assets — both Fungible and Non-Fungible Tokens (NFTs).
* [Contracts Pallet](https://github.com/paritytech/substrate/tree/master/frame/contracts), enable WebAssembly smart-contracts executions.
* [ink!](https://paritytech.github.io/ink/), an eDSL to write smart contracts in Rust for blockchains built on the Substrate framework.

Altogether those technologies enable an array of exciting cross-chain applications & services:

![XCM use cases](/docs/assets/xcm-use-cases.png)


This repository contains the source code of **Trappist**, a feature-rich parachain for exploring and learning about cross-chain applications and services, along with a script to run a complete local multi-chain environment that includes:
* Rococo relay-chain
* Statemine common good asset parachain
* Trappist feature-rich parachain
* An additional parachain capable to execute ink! smart contracts.

All these pre-configured to allow cross-chain communication via XCM messages on HRMP channels.

![Trappist topology](/docs/assets/trappist-topology.png)

### Why "Trappist" ?

The term **Trappist** refers to a [style of beers](https://en.wikipedia.org/wiki/Trappist_beer) brewed in Abbeys by Trappist monks, and is generally associated with authenticity, craftsmanship, integrity and tradition. Aside from any religious consideration, we like to think we put as much care in crafting Blockchain software as monks brewing high-quality beer 🍺.

As Trappist breweries are not intended to be profit-making ventures, this project is non-commercial, open-source software focused solely on experimentation and knowledge sharing with people interested in learning about decentralized technologies.

## Getting Started

Follow the steps below to get started.

### Build Trappist collator

#### Using Nix

Install [nix](https://nixos.org/) and optionally [direnv](https://github.com/direnv/direnv) and
[lorri](https://github.com/target/lorri) for a fully plug and play experience for setting up the
development environment. To get all the correct dependencies activate direnv `direnv allow` and
lorri `lorri shell`.

#### Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).


#### Build

Use the following command to build the Trappist collator binary:

```bash 
cargo b -r --features with-trappist-runtime
cargo b -r --no-default-features --features with-stout-runtime --target-dir target_stout
```

Alternatively, run:
```bash  
./scripts/build_runtimes.sh
```


### XCM Playground via Zombienet

Create a `bin` directory into the root of this repository and place the following binaries inside of it:
- `polkadot` (which you can download from [the releases](https://github.com/paritytech/polkadot/releases))
- `polkadot-parachain` (which you will build from [cumulus](https://github.com/paritytech/cumulus))

Download the [latest release of zombienet](https://github.com/paritytech/zombienet/releases/) into the root of this repository and make it executable:
```bash
$ chmod +x zombienet-linux # OR
$ chmod +x zombienet-macos
```

Then, start the **Trappist** playground with:
```bash
./zombienet-linux -p native spawn ./zombienet/trappist_rococo.toml
```
You can also run:
```bash
# To start Trappist and Stout together
./zombienet-linux -p native spawn ./zombienet/full_network.toml

# To only run stout
./zombienet-linux -p native spawn ./zombienet/stout_rococo.toml
```

### Integration Tests
[parachains-integration-tests](https://github.com/paritytech/parachains-integration-tests) is a tool meant for XCM message execution in a locally spawned network. Tests are written as YAML files and converted into [Mocha](https://mochajs.org/) tests with [Chai](https://www.chaijs.com/) assertions.

The [integration-tests](./integration-tests) directory has tests on Trappist use cases and instructions on how to run them.

### XCM Simulator
The [XCM simulator](./xcm-simulator) can be used to further explore XCM message execution across the various runtimes used by Trappist.
Each Trappist use case is written as a Rust unit test, allowing interactive debugging/exploration of message flows and instruction execution.
Each `execute_with` closure scope within a test can be considered as a block on the corresponding chain, with messages being dispatched to the destination chains via a mock message queue as the closure goes out of scope.
All XCM-specific traces from the interactions are also collected in a single place for easier inspection.

You can run all tests with:
```
cd xcm-simulator && cargo test --release tests::; cd ..
```

## License

Trappist is licensed under [Apache 2](LICENSE).
