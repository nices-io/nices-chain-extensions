#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::Environment;
use ink_lang as ink;

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum NicesExtErr {
    FailGetRandomSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum NicesEnvironment {}

impl Environment for NicesEnvironment {
    const MAX_EVENT_TOPICS: usize =
        <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type ChainExtension = dyn NicesExt;

    type RentFraction = <ink_env::DefaultEnvironment as Environment>::RentFraction;
}

#[ink::chain_extension]
pub trait NicesExt {
    type ErrorCode = NicesExtErr;

    #[ink(extension = 10001, returns_result = false)]
    fn next_u32(max: u32) -> u32;
}