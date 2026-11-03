//! Behaviour every correct version must keep. Runs against the vulnerable and
//! the fixed program alike, and against any candidate patch.

mod common;

use common::{world, TestAccount, TICKET_RENT, TREASURY_BALANCE};
use solana_program::pubkey::Pubkey;
use ssb_0006::{process_instruction, redeem_data, PRIZE};

#[test]
fn holder_redeems_prize_and_rent() {
    let mut w = world();
    w.holder.is_signer = true;
    let accounts = [
        w.ticket.info(),
        w.holder.info(),
        w.treasury.info(),
        w.destination.info(),
    ];
    process_instruction(&w.program_id, &accounts, &redeem_data()).unwrap();
    drop(accounts);
    assert_eq!(w.treasury.lamports, TREASURY_BALANCE - PRIZE);
    assert_eq!(w.ticket.lamports, 0);
    assert_eq!(w.destination.lamports, PRIZE + TICKET_RENT);
}

#[test]
fn unsigned_holder_is_rejected() {
    let mut w = world();
    let accounts = [
        w.ticket.info(),
        w.holder.info(),
        w.treasury.info(),
        w.destination.info(),
    ];
    assert!(process_instruction(&w.program_id, &accounts, &redeem_data()).is_err());
}

#[test]
fn someone_else_cannot_redeem_the_ticket() {
    let mut w = world();
    let mut stranger =
        TestAccount::new(Pubkey::new_unique(), Pubkey::default(), 0, vec![]).signer();
    let accounts = [
        w.ticket.info(),
        stranger.info(),
        w.treasury.info(),
        w.destination.info(),
    ];
    assert!(process_instruction(&w.program_id, &accounts, &redeem_data()).is_err());
}

#[test]
fn forged_ticket_is_rejected() {
    let mut w = world();
    w.holder.is_signer = true;
    w.ticket.owner = Pubkey::new_unique();
    let accounts = [
        w.ticket.info(),
        w.holder.info(),
        w.treasury.info(),
        w.destination.info(),
    ];
    assert!(process_instruction(&w.program_id, &accounts, &redeem_data()).is_err());
}
