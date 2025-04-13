<div align="center">

# 🔐 vOPRF-ID

*A secure nullifiers using verifiable Oblivious Pseudorandom Functions*

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)    [![Tests](https://github.com/privacy-scaling-explorations/voprf-id/actions/workflows/build-test-fmt.yml/badge.svg)](https://github.com/privacy-scaling-explorations/voprf-id/actions/workflows/build-test-fmt.yml)

</div>

## 📝 Overview

vOPRF-ID is a monorepo containing implementations for secure nullifier generation based on verifiable Oblivious Pseudorandom Functions (vOPRF).

You can read more about the protocol in our [docs](https://privacy-scaling-explorations.github.io/vOPRF-ID/)

## 👨‍💻 How to run

This monorepo contains implementation of all necessary components:
* [X] [vOPRF MPC Node implementation](./packages/mpc/)
* [X] [Client Side zk Circuits for verifiability](./packages/zk/)
* [X] [Registry smart-contract for OPRF nodes](./packages/registry/)
* [X] [Optional check script for OPRF nodes](./packages/scripts/)

You can run a vOPRF MPC node either locally or in a docker container.

### To run locally

#### 0. Deploy a registry smart-contract
This step is optional, as you can reuse an already deployed contract, where OPRF nodes can store their public keys.

If you want to deploy your own contract:

```bash
cd packages/registry
```

Then create the .env file, and fill it as described in [.env.example](/packages/registry/.env.example).

To deploy, run:

```bash
forge script script/Registry.s.sol:RegistryScript --rpc-url "<YOUR_RPC_URL>" --broadcast --private-key "<YOUR_PRIVATE_KEY>" -vvvv
```

Get the deployed contract address, as we'll use it to run OPRF nodes.

#### 1. Initialize phase

First, you need to create an .env file as described in [.env.example](./.env.example).
For the "REGISTRY_ADDRESS" field - you can use one of the deployed contracts.
You should create the .env file in the project root (as well as run other commands).

```bash
cargo run --release -- initialize # This will create an OPRF node private key, store the public key in the registry, and save the private key to a file
```

#### 2. Run the node

To run the node:

```bash
cargo run --release -- serve # This will run a vOPRF node
```

---

### ZK part

ZK gives us verifiability of OPRF output. You can see how it works in a [protocol overview](https://privacy-scaling-explorations.github.io/vOPRF-ID/overview.html).
The project contains two separate zk circuits as it's described in the protocol.
To run the first one you have to fill the [Prover.toml](./packages/zk/oprf_commitment/Prover.toml) file first.
Then, to generate witness, vkey and zk proof:

```bash
cd packages/zk/oprf_commitment
nargo execute
bb prove -b ./target/oprf_commitment.json -w ./target/oprf_commitment.gz -o ./target/proof
bb write_vk -b ./target/oprf_commitment.json -o ./target
bb verify -k ./target/vk -p ./target/proof
```

You have to put vkey to the [/generated](./packages/mpc/generated/) dir, so that OPRF nodes can verify the first circuit.

You can send the generated proof to the vOPRF node using the [testing script](./packages/scripts/test_api.py):

```bash
cd packages/scripts
python3 -m venv .venv
source .venv/bin/activate
pip3 install requests
python3 test_api.py --address localhost --port 8080
```

--

To generate a final zk proof that contains `nullifier` - you have to fill the inputs in the [Prover.toml](./packages/zk/oprf_nullifier/Prover.toml) file.
And run:

```bash
cd packages/zk/oprf_nullifier
nargo execute
bb prove -b ./target/oprf_nullifier.json -w ./target/oprf_nullifier.gz -o ./target/proof
bb write_vk -b ./target/oprf_nullifier.json -o ./target
bb verify -k ./target/vk -p ./target/proof
```

## 👀 P.S

Current version slightly differs from the specification, for example - using Secp256k1 instead of BabyJubJub. In subsequent versions, the implementation will be updated and improved for usability (as well as benchmarks).
