use rand::{rngs::OsRng, RngCore};

use crate::seeders::{Seed, Seeder};

pub struct ExternalLibSeeder;

impl Seeder for ExternalLibSeeder {
    fn seed(&mut self) -> Seed {
        let mut arr = [0u8; 16];
        OsRng.fill_bytes(&mut arr[..]);

        let value = u128::from_ne_bytes(arr);

        Seed(value)
    }
}
