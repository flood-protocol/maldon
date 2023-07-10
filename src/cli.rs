use std::{error, fmt, ops::Deref, str::FromStr};

use clap::Parser;

use alloy_primitives::{Address, FixedBytes};

/// Pattern.
#[derive(Debug, Clone)]
pub struct Pattern(Box<str>);

impl Pattern {
    /// Returns the byte representation of the pattern. Pads the pattern with 0s if it is uneven.
    ///
    /// # Errors
    ///
    /// Returns errors if the conversion fails.
    pub(crate) fn into_bytes(self) -> Result<Vec<u8>, hex::FromHexError> {
        let mut string = self.to_string();

        if self.len() % 2 != 0 {
            string += "0"
        };

        hex::decode(string)
    }
}

impl Deref for Pattern {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Pattern errors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatternError {
    /// The pattern's length exceeds 39 characters or the pattern is empty.
    InvalidPatternLength,
    /// The patters is not in hexadecimal format.
    NonHexPattern,
}

impl fmt::Display for PatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatternError::InvalidPatternLength => write!(f, "invalid length"),
            PatternError::NonHexPattern => write!(f, "pattern must be hex digits only"),
        }
    }
}

impl error::Error for PatternError {}

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
