//! SSB-0004: a savings program that keeps each user's lamports in a vault
//! account at the program address derived from `["vault", owner]`.
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

/// Seed prefix of every vault address.
pub const VAULT_SEED: &[u8] = b"vault";

/// Instruction tag for `Withdraw { amount: u64 }`.
pub const IX_WITHDRAW: u8 = 0;

/// Address of the vault that belongs to `owner`.
pub fn vault_address(owner: &Pubkey, program_id: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[VAULT_SEED, owner.as_ref()], program_id).0
}

/// Encodes a `Withdraw` instruction.
pub fn withdraw_data(amount: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(9);
    data.push(IX_WITHDRAW);
    data.extend_from_slice(&amount.to_le_bytes());
    data
}

#[cfg(target_os = "solana")]
solana_program::entrypoint!(process_instruction);
