#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude::vec::Vec;
use nices_extensions::*;

#[ink::contract(env = NicesEnvironment)]
mod contract {
    use super::*;

    #[ink(storage)]
    pub struct Random {}

    impl Random {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn random_number(&self, max: u32) -> u32 {
            self.env().extension().random_number(max)
        }
    }
}