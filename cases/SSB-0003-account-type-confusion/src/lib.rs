//! SSB-0003: a treasury program that stores two account types with the same
//! layout, a config naming the admin and a user profile naming its user.
//!
//! The crate builds the vulnerable program by default. Enable the `fixed`
//! feature to build the reference fix instead.

#[cfg(not(feature = "fixed"))]
#[path = "vulnerable.rs"]
mod program;

#[cfg(feature = "fixed")]
#[path = "fixed.rs"]
mod program;

pub use program::process_instruction;

/// Size of both account types: a 1-byte type tag followed by a 32-byte address.
pub const ACCOUNT_LEN: usize = 33;

/// Type tag of an account that has not been initialized yet.
pub const TAG_UNINITIALIZED: u8 = 0;
/// Type tag of the config account. Its address field is the admin.
pub const TAG_CONFIG: u8 = 1;
/// Type tag of a user profile. Its address field is the user.
pub const TAG_PROFILE: u8 = 2;

/// Instruction tag for `Payout { amount: u64 }`.
pub const IX_PAYOUT: u8 = 0;
/// Instruction tag for `Register`, which anyone can call to create a profile.
pub const IX_REGISTER: u8 = 1;

/// Encodes a `Payout` instruction.
pub fn payout_data(amount: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(9);
    data.push(IX_PAYOUT);
    data.extend_from_slice(&amount.to_le_bytes());
    data
}

/// Encodes a `Register` instruction.
pub fn register_data() -> Vec<u8> {
    vec![IX_REGISTER]
}

/// Account data for an initialized account of type `tag` holding `key`.
pub fn account_data(tag: u8, key: &solana_program::pubkey::Pubkey) -> Vec<u8> {
    let mut data = Vec::with_capacity(ACCOUNT_LEN);
    data.push(tag);
    data.extend_from_slice(key.as_ref());
    data
}

#[cfg(target_os = "solana")]
solana_program::entrypoint!(process_instruction);
