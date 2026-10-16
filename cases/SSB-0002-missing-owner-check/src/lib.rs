//! SSB-0002: a treasury whose payout instruction reads its admin from a config
//! account without checking who owns that account.
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

/// Size of the config account data: the 32-byte admin address.
pub const CONFIG_LEN: usize = 32;

/// Instruction tag for `Payout { amount: u64 }`.
pub const IX_PAYOUT: u8 = 0;

/// Encodes a `Payout` instruction.
pub fn payout_data(amount: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(9);
    data.push(IX_PAYOUT);
    data.extend_from_slice(&amount.to_le_bytes());
    data
}

#[cfg(target_os = "solana")]
solana_program::entrypoint!(process_instruction);
