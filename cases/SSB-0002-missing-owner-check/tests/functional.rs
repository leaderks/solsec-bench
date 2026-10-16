//! Behaviour every correct version must keep. Runs against the vulnerable and
//! the fixed program alike, and against any candidate patch.

mod common;

use common::{world, TestAccount, TREASURY_BALANCE};
use solana_program::pubkey::Pubkey;
use ssb_0002_missing_owner_check::{payout_data, process_instruction};

#[test]
fn admin_can_pay_out() {
    let mut w = world();
    w.admin.is_signer = true;
    let accounts = [
        w.config.info(),
        w.treasury.info(),
        w.admin.info(),
        w.destination.info(),
    ];
    process_instruction(&w.program_id, &accounts, &payout_data(5_000)).unwrap();
    drop(accounts);
    assert_eq!(w.treasury.lamports, TREASURY_BALANCE - 5_000);
    assert_eq!(w.destination.lamports, 5_000);
}

#[test]
fn unsigned_admin_is_rejected() {
    let mut w = world();
    let accounts = [
        w.config.info(),
        w.treasury.info(),
        w.admin.info(),
        w.destination.info(),
    ];
    assert!(process_instruction(&w.program_id, &accounts, &payout_data(1)).is_err());
}

#[test]
fn other_signer_is_rejected() {
    let mut w = world();
    let mut stranger =
        TestAccount::new(Pubkey::new_unique(), Pubkey::default(), 0, vec![]).signer();
    let accounts = [
        w.config.info(),
        w.treasury.info(),
        stranger.info(),
        w.destination.info(),
    ];
    assert!(process_instruction(&w.program_id, &accounts, &payout_data(1)).is_err());
}

#[test]
fn foreign_treasury_is_rejected() {
    let mut w = world();
    w.admin.is_signer = true;
    w.treasury.owner = Pubkey::new_unique();
    let accounts = [
        w.config.info(),
        w.treasury.info(),
        w.admin.info(),
        w.destination.info(),
    ];
    assert!(process_instruction(&w.program_id, &accounts, &payout_data(1)).is_err());
}
