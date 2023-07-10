mod cli;
mod mine;

use clap::Parser;

use cli::Config;

use mine::Miner;

fn main() {
    let config = Config::parse();

    let miner = Miner::new(config);

    let (address, salt) = miner.mine();

    println!("Found salt {salt:?} ==> {address:?}");
}
