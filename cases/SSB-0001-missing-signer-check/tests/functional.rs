//! Behaviour every correct version must keep. Runs against the vulnerable and
//! the fixed program alike, and against any candidate patch.

mod common;

use common::{world, TestAccount, VAULT_BALANCE};
use solana_program::pubkey::Pubkey;
use ssb_0001_missing_signer_check::{process_instruction, withdraw_data};

#[test]
fn authority_can_withdraw() {
    let mut w = world();
    w.authority.is_signer = true;
    let accounts = [w.vault.info(), w.authority.info(), w.destination.info()];
    process_instruction(&w.program_id, &accounts, &withdraw_data(1_000)).unwrap();
    drop(accounts);
    assert_eq!(w.vault.lamports, VAULT_BALANCE - 1_000);
    assert_eq!(w.destination.lamports, 1_000);
}

#[test]
fn wrong_authority_is_rejected() {
    let mut w = world();
    let mut stranger =
        TestAccount::new(Pubkey::new_unique(), Pubkey::default(), 0, vec![]).signer();
    let accounts = [w.vault.info(), stranger.info(), w.destination.info()];
    assert!(process_instruction(&w.program_id, &accounts, &withdraw_data(1)).is_err());
}

#[test]
fn foreign_vault_is_rejected() {
    let mut w = world();
    w.authority.is_signer = true;
    w.vault.owner = Pubkey::new_unique();
    let accounts = [w.vault.info(), w.authority.info(), w.destination.info()];
    assert!(process_instruction(&w.program_id, &accounts, &withdraw_data(1)).is_err());
}

#[test]
fn cannot_overdraw() {
    let mut w = world();
    w.authority.is_signer = true;
    let accounts = [w.vault.info(), w.authority.info(), w.destination.info()];
    let res = process_instruction(&w.program_id, &accounts, &withdraw_data(VAULT_BALANCE + 1));
    assert!(res.is_err());
}
