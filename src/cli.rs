use std::{ops::Deref, str::FromStr};

use alloy_primitives::{Address, FixedBytes};

/// Pattern.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct Pattern(Box<str>);

impl Deref for Pattern {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Pattern {
    /// Returns the byte representation of the pattern. Pads the pattern with 0s if it is uneven.
    ///
    /// # Errors
    ///
    /// Returns errors if the conversion fails.
    pub(super) fn into_bytes(self) -> Result<Vec<u8>, hex::FromHexError> {
        let mut string = self.to_string();

        if self.len() % 2 != 0 {
            string += "0"
        };

        hex::decode(string)
    }
}

/// Pattern errors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub(super) enum PatternError {
    #[error("the pattern's length exceeds 39 characters or the pattern is empty")]
    InvalidPatternLength,
    #[error("the patters is not in hexadecimal format")]
    NonHexPattern,
}

impl FromStr for Pattern {
    type Err = PatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() >= 40 || s.is_empty() {
            return Err(PatternError::InvalidPatternLength);
        }

        if s.chars().any(|c| !c.is_ascii_hexdigit()) {
            return Err(PatternError::NonHexPattern);
        }

        Ok(Self(s.into()))
    }
}

#[derive(Clone, Debug, clap::Parser)]
#[command(
    name = "maldon",
    about = "Maldon is a fast CREATE2 and CREATE3 salt miner."
)]
pub(super) enum Maldon {
    /// Mines a CREATE2 salt.
    Create2 {
        /// Address of the contract deployer.
        deployer: Address,
        /// Address of the Factory contract. Defaults to the Immutable CREATE2 Factory by 0age.
        #[clap(short, long)]
        factory: Option<Address>,
        /// Hash of the initialization code.
        init_code_hash: FixedBytes<32>,
        /// Pattern to search for. Must be hex digits only and between 1 and 20 characters.
        pattern: Pattern,
    },
    /// Mines a CREATE3 salt.
    Create3 {
        /// Address of the contract deployer.
        deployer: Address,
        /// Address of the Factory contract. Defaults to the Aori Factory.
        #[clap(short, long)]
        factory: Option<Address>,
        /// Pattern to search for. Must be hex digits only and between 1 and 20 characters.
        pattern: Pattern,
    },
}

#[test]
fn test_pattern_bytes_from_str() -> Result<(), PatternError> {
    let pattern_bytes = "f100d0".parse::<Pattern>().and_then(|pattern| {
        pattern
            .into_bytes()
            .map_err(|_| PatternError::NonHexPattern)
    })?;

    assert_eq!(&[241, 0, 208], pattern_bytes.as_slice());

    Ok(())
}
