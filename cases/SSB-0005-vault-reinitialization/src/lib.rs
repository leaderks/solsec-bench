//! SSB-0005: a vault program. `Initialize` records the authority of a fresh
//! vault account, and `Withdraw` lets that authority move lamports out.
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

/// Vault data: a 1-byte state followed by the 32-byte authority.
pub const VAULT_LEN: usize = 33;
/// State byte of a vault that has not been initialized.
pub const STATE_UNINITIALIZED: u8 = 0;
/// State byte of an initialized vault.
pub const STATE_INITIALIZED: u8 = 1;

/// Instruction tag for `Initialize`.
pub const IX_INITIALIZE: u8 = 0;
/// Instruction tag for `Withdraw { amount: u64 }`.
pub const IX_WITHDRAW: u8 = 1;

/// Encodes an `Initialize` instruction.
pub fn initialize_data() -> Vec<u8> {
    vec![IX_INITIALIZE]
}

/// Encodes a `Withdraw` instruction.
pub fn withdraw_data(amount: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(9);
    data.push(IX_WITHDRAW);
    data.extend_from_slice(&amount.to_le_bytes());
    data
}

/// Data of a vault initialized with `authority`.
pub fn initialized_vault(authority: &Pubkey) -> Vec<u8> {
    let mut data = Vec::with_capacity(VAULT_LEN);
    data.push(STATE_INITIALIZED);
    data.extend_from_slice(authority.as_ref());
    data
}

#[cfg(target_os = "solana")]
solana_program::entrypoint!(process_instruction);
