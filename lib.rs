#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::{DefaultEnvironment, Environment};
use ink_lang as ink;

pub enum NicesEnvironment {}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ErrorCode {
    ExtError,
}

impl ink_env::chain_extension::FromStatusCode for ErrorCode {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::ExtError),
            _ => panic!("encountered unknown status code"),
        }
    }
}

impl Environment for NicesEnvironment {
    const MAX_EVENT_TOPICS: usize = <DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <DefaultEnvironment as Environment>::AccountId;
    type Balance = <DefaultEnvironment as Environment>::Balance;
    type Hash = <DefaultEnvironment as Environment>::Hash;
    type Timestamp = <DefaultEnvironment as Environment>::Timestamp;
    type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
    type ChainExtension = NicesExt;

    type RentFraction = <DefaultEnvironment as Environment>::RentFraction;
}

#[ink::chain_extension]
pub trait NicesExt {
    type ErrorCode = ErrorCode;

    #[ink(extension = 10000, returns_result = false)]
    fn next_u32(max: u32) -> u32;
}