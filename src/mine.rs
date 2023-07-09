use crate::cli::{Config, Pattern};
use alloy_primitives::{keccak256, Address, FixedBytes};
use rand::{thread_rng, Rng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

/// The maximum value of the nonce segment of the salt. Since this is 6 bytes, we shift the u64 max by 2 bytes.
const MAX_INCREMENTING_NONCE: u64 = u64::MAX >> 2;

pub struct Miner {
    hash_buffer: [u8; 85],
    pattern_bytes: Vec<u8>,
}

impl Miner {
    pub fn new(config: Config) -> Self {
        let pattern_bytes = pattern_bytes_from_str(config.pattern);
        let mut hash_buffer = [0u8; 85];
        hash_buffer[0] = 0xff;
        // header: 0xff + factory + caller + salt_random_segment + nonce_segment + init_code_hash
        hash_buffer[1..21].copy_from_slice(config.factory.as_slice());
        hash_buffer[21..41].copy_from_slice(config.caller.as_slice());
        // bytes 53..85 are the init_code_hash
        hash_buffer[53..85].copy_from_slice(config.init_code_hash.as_slice());

        Self {
            hash_buffer,
            pattern_bytes,
        }
    }

    /// Given a Config object with a factory address, a caller address, and a
    /// keccak-256 hash of the contract initialization code, search for salts that
    /// will enable the factory contract to deploy a contract to a gas-efficient
    /// address via CREATE2.
    ///
    /// The 32-byte salt is constructed as follows:
    ///   - the 20-byte calling address (to prevent frontrunning)
    ///   - a random 6-byte segment (to prevent collisions with other runs)
    ///   - a 6-byte nonce segment (incrementally stepped through during the run)
    ///
    /// When a salt that will result in the creation of a gas-efficient contract
    /// address is found, it will be appended to `efficient_addresses.txt` along
    /// with the resultant address and the "value" (i.e. approximate rarity) of the
    /// resultant address.
    pub fn mine(&mut self) -> (Address, FixedBytes<32>) {
        let mut rng = thread_rng();

        loop {
            // Puts 6 random bytes into the first 6 bytes of salt. Bytes 41..47 are the salt_random_segment
            rng.fill(self.hash_buffer[41..47].as_mut());

            let maybe_found = (0..MAX_INCREMENTING_NONCE).into_par_iter().find_map_any(
                |salt_incremented_nonce| {
                    let mut to_hash = self.hash_buffer;
                    // bytes 47..53 are the nonce_segment
                    to_hash[47..53].copy_from_slice(&salt_incremented_nonce.to_be_bytes()[2..]);

                    let hash = keccak256(to_hash);
                    // check wether we have a match
                    if hash[12..].starts_with(&self.pattern_bytes) {
                        Some((
                            Address::from_slice(&hash[12..32]),
                            FixedBytes::<32>::from_slice(&to_hash[21..53]),
                        ))
                    } else {
                        None
                    }
                },
            );

            if let Some(found) = maybe_found {
                return found;
            }
        }
    }
}

/// Given a pattern string, returns its byte representation. Pads the pattern with 0s if it is uneven.
///
/// # Arguments
///
/// * `pattern` - The pattern string to convert to bytes.
///
/// # Returns
/// The byte representation of the pattern.
fn pattern_bytes_from_str(pattern: Pattern) -> Vec<u8> {
    let padded_pattern = if pattern.len() % 2 != 0 {
        pattern.to_string() + "0"
    } else {
        pattern.to_string()
    };

    hex::decode(padded_pattern).unwrap()
}

#[test]
fn test_pattern_bytes_from_str() {
    use std::str::FromStr;
    let pattern = Pattern::from_str("f100d0").unwrap();

    let pattern_bytes = pattern_bytes_from_str(pattern);
    println!("{:?}", pattern_bytes);
}
