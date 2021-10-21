# Trappist

**Trappist** is a versatile Proof-of-Authority (PoA) Blockchain software that supports hosting decentralized applications (dApps), including Smart Contracts, Crypto-tokens & Non-Fungible
Tokens (NFTs).

This repository contains the source code of the Trappist Blockchain node & runtime, based on
[Substrate](https://www.substrate.dev/), an open-source framework for building tailored Blockchain solutions.

The term **Trappist** refers to a [style of beers](https://en.wikipedia.org/wiki/Trappist_beer) brewed in Abbeys by Trappist monks, and is generally associated with authenticity, craftsmanship, integrity and tradition. Aside from any religious consideration, we like to think we put as much care in crafting Blockchain software as monks brewing high-quality beer 🍺.

As Trappist breweries are not intended to be profit-making ventures, this project is non-commercial, open-source software focused solely on experimentation and knowledge sharing with people interested in learning about decentralized technologies.

## Getting Started

Follow the steps below to get started.

### Using Nix

Install [nix](https://nixos.org/) and optionally [direnv](https://github.com/direnv/direnv) and
[lorri](https://github.com/target/lorri) for a fully plug and play experience for setting up the
development environment. To get all the correct dependencies activate direnv `direnv allow` and
lorri `lorri shell`.

### Rust Setup

First, complete the [basic Rust setup instructions](./doc/rust-setup.md).

### Run

Use Rust's native `cargo` command to build and launch the Trappist node:

```sh
cargo run --release -- --dev --tmp
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/trappist -h
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/trappist --dev
```

Purge the development chain's state:

```bash
./target/release/trappist purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/trappist -lruntime=debug --dev
```

### Connect with Polkadot-JS Apps Front-end

Once the node template is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your
local node template.

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, refer to our
[Start a Private Network tutorial](https://substrate.dev/docs/en/tutorials/start-a-private-network/).

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command (`cargo build --release && ./target/release/trappist --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/trappist --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/trappist purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```

## License

Trappist is licensed under [Apache 2](LICENSE).
