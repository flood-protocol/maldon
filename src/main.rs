mod cli;
mod mine;

use clap::Parser;

use alloy_primitives::{address, Address};

use {
    cli::Maldon,
    mine::{Create2Miner, Create3Miner, Miner},
};

const CREATE2_DEFAULT_FACTORY: Address = address!("0000000000ffe8b47b3e2130213b802212439497");

const CREATE3_DEFAULT_FACTORY: Address = address!("2dfcc7415d89af828cbef005f0d072d8b3f23183");

fn main() -> Result<(), hex::FromHexError> {
    let (address, salt) = match Maldon::parse() {
        Maldon::Create2 {
            deployer,
            factory,
            init_code_hash,
            pattern,
        } => {
            let factory = factory.unwrap_or(CREATE2_DEFAULT_FACTORY);

            Create2Miner::new(factory, deployer, init_code_hash).mine(&pattern.into_bytes()?)
        }
        Maldon::Create3 {
            deployer,
            factory,
            pattern,
        } => {
            let factory = factory.unwrap_or(CREATE3_DEFAULT_FACTORY);

            Create3Miner::new(factory, deployer).mine(&pattern.into_bytes()?)
        }
    };

    println!("Found salt {salt:?} ==> {address:?}");

    Ok(())
}
