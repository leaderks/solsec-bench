//! SSB-0006: a prize program. Each ticket account names its holder and can be
//! redeemed once for a fixed prize, after which the ticket is closed and its
//! rent refunded to the holder.
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

/// Ticket data: a 1-byte state followed by the 32-byte holder.
pub const TICKET_LEN: usize = 33;
/// State byte of a ticket that can be redeemed.
pub const STATE_VALID: u8 = 1;
/// Lamports paid from the treasury for one ticket.
pub const PRIZE: u64 = 1_000_000_000;

/// Instruction tag for `Redeem`.
pub const IX_REDEEM: u8 = 0;

/// Encodes a `Redeem` instruction.
pub fn redeem_data() -> Vec<u8> {
    vec![IX_REDEEM]
}

/// Data of a valid ticket held by `holder`.
pub fn valid_ticket(holder: &Pubkey) -> Vec<u8> {
    let mut data = Vec::with_capacity(TICKET_LEN);
    data.push(STATE_VALID);
    data.extend_from_slice(holder.as_ref());
    data
}

#[cfg(target_os = "solana")]
solana_program::entrypoint!(process_instruction);
