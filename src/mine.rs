use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use rand::{thread_rng, Rng};

use alloy_primitives::{keccak256, Address, FixedBytes};

pub trait Miner {
    /// Runs the Miner.
    fn mine(&self, pattern: &[u8]) -> (Address, FixedBytes<32>);
}

/// A CREATE2 Miner.
///
/// Given a Config object with a factory address, a caller address, and a
/// keccak-256 hash of the contract initialization code, search for salts that
/// will enable the factory contract to deploy a contract to a gas-efficient
/// address via CREATE2.
///
/// The 32-byte salt is constructed as follows:
///   * the 20-byte calling address (to prevent frontrunning)
///   * a random 6-byte segment (to prevent collisions with other runs)
///   * a 6-byte nonce segment (incrementally stepped through during the run)
///
/// When a salt that will result in the creation of a gas-efficient contract
/// address is found, it will be appended to `efficient_addresses.txt` along
/// with the resultant address and the "value" (i.e. approximate rarity) of the
/// resultant address.
#[derive(Debug, Clone, Copy)]
pub struct Create2Miner {
    factory: Address,
    deployer: Address,
    init_code_hash: FixedBytes<32>,
}

impl Create2Miner {
    /// The maximum value of the nonce segment of the salt. Since this is 6 bytes, we shift the u64 max by 2 bytes.
    const MAX_INCREMENTING_NONCE: u64 = u64::MAX >> 2;

    /// Creates a new CREATE2 miner.
    ///
    /// # Arguments
    ///
    /// `factory` - CREATE2 factory address.
    ///
    /// `deployer` - Deployer address.
    ///
    /// `init_code_hash` - Keccak-256 hash of the contract initialization code.
    pub fn new(factory: Address, deployer: Address, init_code_hash: FixedBytes<32>) -> Self {
        Self {
            factory,
            deployer,
            init_code_hash,
        }
    }
}

impl Miner for Create2Miner {
    fn mine(&self, pattern: &[u8]) -> (Address, FixedBytes<32>) {
        let mut rng = thread_rng();
        let mut hash_buffer = [0u8; 85];
        hash_buffer[0] = 0xff;
        // header: 0xff + factory + deployer + salt_random_segment + nonce_segment + init_code_hash
        hash_buffer[1..21].copy_from_slice(self.factory.as_slice());
        hash_buffer[21..41].copy_from_slice(self.deployer.as_slice());
        // bytes 53..85 are the init_code_hash
        hash_buffer[53..85].copy_from_slice(self.init_code_hash.as_slice());

        loop {
            // Puts 6 random bytes into the first 6 bytes of salt. Bytes 41..47 are the salt_random_segment
            rng.fill(hash_buffer[41..47].as_mut());

            let maybe_found = (0..Self::MAX_INCREMENTING_NONCE)
                .into_par_iter()
                .find_map_any(move |salt_incremented_nonce| {
                    let mut to_hash = hash_buffer;
                    // bytes 47..53 are the nonce_segment
                    to_hash[47..53].copy_from_slice(&salt_incremented_nonce.to_be_bytes()[2..]);

                    let hash = keccak256(to_hash);
                    // check wether we have a match
                    hash[12..].starts_with(pattern).then(|| {
                        (
                            Address::from_slice(&hash[12..32]),
                            FixedBytes::<32>::from_slice(&to_hash[21..53]),
                        )
                    })
                });

            // Exists the loop.
            if let Some(found) = maybe_found {
                break (found);
            }
        }
    }
}
