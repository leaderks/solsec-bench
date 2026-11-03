//! Behaviour every correct version must keep. Runs against the vulnerable and
//! the fixed program alike, and against any candidate patch.

mod common;

use common::{world, TestAccount, ALICE_BALANCE, BOB_BALANCE};
use solana_program::pubkey::Pubkey;
use ssb_0007::{balance_of, process_instruction, transfer_data};

#[test]
fn owner_can_transfer_to_another_ledger() {
    let mut w = world();
    w.alice.is_signer = true;
    let accounts = [w.alice_ledger.info(), w.bob_ledger.info(), w.alice.info()];
    process_instruction(&w.program_id, &accounts, &transfer_data(300)).unwrap();
    drop(accounts);
    assert_eq!(balance_of(&w.alice_ledger.data), ALICE_BALANCE - 300);
    assert_eq!(balance_of(&w.bob_ledger.data), BOB_BALANCE + 300);
}

#[test]
fn unsigned_owner_is_rejected() {
    let mut w = world();
    let accounts = [w.alice_ledger.info(), w.bob_ledger.info(), w.alice.info()];
    assert!(process_instruction(&w.program_id, &accounts, &transfer_data(1)).is_err());
}

#[test]
fn cannot_spend_someone_elses_ledger() {
    let mut w = world();
    w.alice.is_signer = true;
    let accounts = [w.bob_ledger.info(), w.alice_ledger.info(), w.alice.info()];
    assert!(process_instruction(&w.program_id, &accounts, &transfer_data(1)).is_err());
}

#[test]
fn cannot_transfer_more_than_the_balance() {
    let mut w = world();
    w.alice.is_signer = true;
    let accounts = [w.alice_ledger.info(), w.bob_ledger.info(), w.alice.info()];
    assert!(
        process_instruction(&w.program_id, &accounts, &transfer_data(ALICE_BALANCE + 1)).is_err()
    );
}

#[test]
fn forged_ledger_is_rejected() {
    let mut w = world();
    w.alice.is_signer = true;
    let mut forged = TestAccount::new(
        Pubkey::new_unique(),
        Pubkey::new_unique(),
        1_000_000,
        w.alice_ledger.data.clone(),
    );
    let accounts = [forged.info(), w.bob_ledger.info(), w.alice.info()];
    assert!(process_instruction(&w.program_id, &accounts, &transfer_data(1)).is_err());
}
