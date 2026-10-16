//! SSB-0001: a lamport vault whose withdraw instruction trusts the caller to
//! supply the right authority account.
//!
//! The crate builds the vulnerable program by default. Enable the `fixed`
//! feature to build the reference fix instead. Both expose the same API so the
//! same tests can run against either version.

#[cfg(not(feature = "fixed"))]
#[path = "vulnerable.rs"]
mod program;

#[cfg(feature = "fixed")]
#[path = "fixed.rs"]
mod program;

pub use program::process_instruction;

/// Size of the vault account data: the 32-byte authority address.
pub const VAULT_LEN: usize = 32;

/// Instruction tag for `Withdraw { amount: u64 }`.
pub const IX_WITHDRAW: u8 = 0;

/// Encodes a `Withdraw` instruction.
pub fn withdraw_data(amount: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(9);
    data.push(IX_WITHDRAW);
    data.extend_from_slice(&amount.to_le_bytes());
    data
}

#[cfg(target_os = "solana")]
solana_program::entrypoint!(process_instruction);
