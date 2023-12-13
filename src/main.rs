mod cli;
mod mine;

use clap::Parser;

use cli::Config;

use mine::{Create2Miner, Create3Miner, Miner};

const CREATE2_DEFAULT_FACTORY: [u8; 20] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xe8, 0xB4, 0x7B, 0x3e, 0x21, 0x30, 0x21, 0x3B, 0x80, 0x22,
    0x12, 0x43, 0x94, 0x97,
];

const CREATE3_DEFAULT_FACTORY: [u8; 20] = [
    0x2D, 0xfc, 0xc7, 0x41, 0x5D, 0x89, 0xaf, 0x82, 0x8c, 0xbe, 0xf0, 0x05, 0xF0, 0xd0, 0x72, 0xD8,
    0xb3, 0xF2, 0x31, 0x83,
];

fn main() {
    let config = Config::parse();
    let pattern = config.pattern.into_bytes().expect("pattern is valid");

    let (address, salt) = if config.create3 {
        let factory = if let Some(factory) = config.factory {
            factory
        } else {
            CREATE3_DEFAULT_FACTORY.into()
        };

        Create3Miner::new(factory, config.deployer).mine(&pattern)
    } else {
        let factory = if let Some(factory) = config.factory {
            factory
        } else {
            CREATE2_DEFAULT_FACTORY.into()
        };

        Create2Miner::new(
            factory,
            config.deployer,
            config
                .init_code_hash
                .expect("init code hash must be present for CREATE2."),
        )
        .mine(&pattern)
    };

    println!("Found salt {salt:?} ==> {address:?}");
}
