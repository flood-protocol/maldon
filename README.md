# Maldon 🧂⛏️

Maldon is a CLI for quickly finding salts that create pattern matching Ethereum addresses via CREATE2 or CREATE3.
Written in Rust with [Alloy](https://github.com/alloy-rs/core).

Maldon is heavely inspired by [Create2Crunch](https://github.com/0age/create2crunch), with the difference that it supports arbitrary patterns, will exit once it finds a salt and with additional CREATE3 support.
Create2Crunch is still the better choice if you need GPU support, don't have a predermined pattern in mind or don't care about CREATE3.

## Installation

```bash
git clone https://github.com/flood-protocol/maldon.git
cd maldon
# Run it directly
cargo run --release -- create2 --factory <FACTORY> <CALLER> <INIT_CODE_HASH> <PATTERN>

# Add it to your path
cargo install --path .
```

## Usage

```
Usage: maldon <COMMAND>

Commands:
  create2  Mines a CREATE2 salt
  create3  Mines a CREATE3 salt
  help     Print this message or the help of the given subcommand(s)

Usage: maldon create2 [OPTIONS] <DEPLOYER> <INIT_CODE_HASH> <PATTERN>

Arguments:
  <DEPLOYER>        Address of the contract deployer
  <INIT_CODE_HASH>  Hash of the initialization code
  <PATTERN>         Pattern to search for. Must be hex digits only and between 1 and 20 characters

Options:
  -f, --factory <FACTORY>  Address of the Factory contract. Defaults to the Immutable CREATE2 Factory by 0age
  -h, --help               Print help

Usage: maldon create3 [OPTIONS] <DEPLOYER> <PATTERN>

Arguments:
  <DEPLOYER>  Address of the contract deployer
  <PATTERN>   Pattern to search for. Must be hex digits only and between 1 and 20 characters

Options:
  -f, --factory <FACTORY>  Address of the Factory contract. Defaults to the Aori Factory
  -h, --help               Print help
```
