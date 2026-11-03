//! Behaviour every correct version must keep. Runs against the vulnerable and
//! the fixed program alike, and against any candidate patch.

mod common;

use common::{world, TestAccount, VAULT_BALANCE};
use solana_program::pubkey::Pubkey;
use ssb_0004::{process_instruction, vault_address, withdraw_data};

#[test]
fn owner_can_withdraw_from_own_vault() {
    let mut w = world();
    w.owner.is_signer = true;
    let accounts = [w.vault.info(), w.owner.info(), w.destination.info()];
    process_instruction(&w.program_id, &accounts, &withdraw_data(7_000)).unwrap();
    drop(accounts);
    assert_eq!(w.vault.lamports, VAULT_BALANCE - 7_000);
    assert_eq!(w.destination.lamports, 7_000);
}

#[test]
fn unsigned_owner_is_rejected() {
    let mut w = world();
    let accounts = [w.vault.info(), w.owner.info(), w.destination.info()];
    assert!(process_instruction(&w.program_id, &accounts, &withdraw_data(1)).is_err());
}

#[test]
fn vault_owned_by_another_program_is_rejected() {
    let mut w = world();
    w.owner.is_signer = true;
    w.vault.owner = Pubkey::new_unique();
    let accounts = [w.vault.info(), w.owner.info(), w.destination.info()];
    assert!(process_instruction(&w.program_id, &accounts, &withdraw_data(1)).is_err());
}

#[test]
fn cannot_withdraw_more_than_the_balance() {
    let mut w = world();
    w.owner.is_signer = true;
    let accounts = [w.vault.info(), w.owner.info(), w.destination.info()];
    assert!(
        process_instruction(&w.program_id, &accounts, &withdraw_data(VAULT_BALANCE + 1)).is_err()
    );
}

#[test]
fn each_user_has_a_distinct_vault() {
    let program_id = Pubkey::new_unique();
    let a = TestAccount::new(Pubkey::new_unique(), Pubkey::default(), 0, vec![]);
    let b = TestAccount::new(Pubkey::new_unique(), Pubkey::default(), 0, vec![]);
    assert_ne!(
        vault_address(&a.key, &program_id),
        vault_address(&b.key, &program_id)
    );
}
