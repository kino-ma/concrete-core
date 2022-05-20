use std::mem;

use stdweb::js;

use crate::seeders::{Seed, Seeder};

pub struct JsSeeder;

impl Seeder for JsSeeder {
    fn seed(&mut self) -> Seed {
        assert_eq!(mem::size_of::<usize>(), 4);

        // panic!("entered to seed");
        let mut arr = [0u8; 16];
        let mut dest = &mut arr[..];
        let ptr = dest.as_mut_ptr() as i32;
        println!("ptr: {}", ptr);

        js! {
            let length = 16;
            let array = new Uint8Array(length);
            self.crypto.getRandomValues(array);
            // let ptr = @{ ptr };
            let ptr = 107;
            HEAPU8.set(array, ptr);
        };

        let value = u128::from_ne_bytes(arr);

        Seed(value)
        // Seed(ptr as u128)
    }
}
