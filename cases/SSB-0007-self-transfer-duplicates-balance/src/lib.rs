//! SSB-0007: an internal ledger. Each ledger account records its owner and a
//! balance, and owners move balance between ledgers with `Transfer`.
//!
//! The crate builds the vulnerable program by default. Enable the `fixed`
//! feature to build the reference fix instead.

use solana_program::pubkey::Pubkey;

#[cfg(not(feature = "fixed"))]
#[path = "vulnerable.rs"]
mod program;

#[cfg(feature = "fixed")]
#[path = "fixed.rs"]
mod program;

pub use program::process_instruction;

/// Ledger data: the 32-byte owner followed by a little-endian u64 balance.
pub const LEDGER_LEN: usize = 40;

/// Instruction tag for `Transfer { amount: u64 }`.
pub const IX_TRANSFER: u8 = 0;

/// Encodes a `Transfer` instruction.
pub fn transfer_data(amount: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(9);
    data.push(IX_TRANSFER);
    data.extend_from_slice(&amount.to_le_bytes());
    data
}

/// Data of a ledger owned by `owner` holding `balance`.
pub fn ledger(owner: &Pubkey, balance: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(LEDGER_LEN);
    data.extend_from_slice(owner.as_ref());
    data.extend_from_slice(&balance.to_le_bytes());
    data
}

/// Reads the balance stored in ledger data.
pub fn balance_of(data: &[u8]) -> u64 {
    u64::from_le_bytes(data[32..LEDGER_LEN].try_into().unwrap())
}

#[cfg(target_os = "solana")]
solana_program::entrypoint!(process_instruction);
