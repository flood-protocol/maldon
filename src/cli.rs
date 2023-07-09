use std::{ops::Deref, str::FromStr};

use alloy_primitives::{Address, FixedBytes};
use clap::Parser;

#[derive(Debug, Clone)]
pub struct Pattern(Box<str>);

impl Deref for Pattern {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Pattern {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() >= 40 || s.is_empty() {
            Err("Invalid length.")
        } else if s.chars().all(|c| c.is_ascii_hexdigit()) {
            Ok(Self(s.into()))
        } else {
            Err("Pattern must be hex digits only.")
        }
    }
}

#[derive(Parser, Debug, Clone)]
pub struct Config {
    /// Address of the CREATE2 Factory contract.
    pub factory: Address,
    /// Address of the contract deployer.
    pub caller: Address,
    /// Hash of the initialization code.
    pub init_code_hash: FixedBytes<32>,
    #[clap(short, long)]
    /// Pattern to search for. Must be hex digits only and between 1 and 20 characters.
    pub pattern: Pattern,
}
