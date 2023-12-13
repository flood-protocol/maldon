mod cli;
mod mine;

use clap::Parser;

use cli::Config;

use mine::{Create2Miner, Miner};

fn main() {
    let config = Config::parse();

    let miner = Create2Miner::new(config.factory, config.deployer, config.init_code_hash);

    let (address, salt) = miner.mine(&config.pattern.into_bytes().expect("pattern is valid"));

    println!("Found salt {salt:?} ==> {address:?}");
}
