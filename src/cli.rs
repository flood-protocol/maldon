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
    /// Flag to enable CREATE3 mining.
    #[clap(short, long, default_value = "false")]
    pub create3: bool,
    /// Address of the Factory contract. Defaults to the Immutable CREATE2 Factory by 0age for CREATE2 and the Aori Factory for CREATE3.
    pub factory: Option<Address>,
    /// Address of the contract deployer.
    #[clap(short, long)]
    pub deployer: Address,
    /// Hash of the initialization code. Only needed for CREATE2.
    #[clap(required_unless_present = "create3")]
    pub init_code_hash: Option<FixedBytes<32>>,
    #[clap(short, long)]
    /// Pattern to search for. Must be hex digits only and between 1 and 20 characters.
    pub pattern: Pattern,
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
