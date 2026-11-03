//! Behaviour every correct version must keep. Runs against the vulnerable and
//! the fixed program alike, and against any candidate patch.

mod common;

use anchor_lang::prelude::Pubkey;
use anchor_lang::InstructionData;
use common::{vault_data, world, TestAccount, VAULT_BALANCE};
use ssb_0008::{entry, instruction, ID};

fn withdraw(amount: u64) -> Vec<u8> {
    instruction::Withdraw { amount }.data()
}

#[test]
fn authority_can_withdraw() {
    let mut w = world();
    w.authority.is_signer = true;
    let expected_data = vault_data(w.authority.key);
    let data = withdraw(8_000);
    let accounts = [w.vault.info(), w.authority.info(), w.destination.info()];
    entry(&ID, &accounts, &data).unwrap();
    assert_eq!(accounts[0].lamports(), VAULT_BALANCE - 8_000);
    assert_eq!(accounts[2].lamports(), 8_000);
    assert_eq!(&accounts[0].data.borrow()[..], &expected_data[..]);
}

#[test]
fn unsigned_authority_is_rejected() {
    let mut w = world();
    let data = withdraw(1);
    let accounts = [w.vault.info(), w.authority.info(), w.destination.info()];
    assert!(entry(&ID, &accounts, &data).is_err());
}

#[test]
fn vault_owned_by_another_program_is_rejected() {
    let mut w = world();
    w.authority.is_signer = true;
    w.vault.owner = Pubkey::new_unique();
    let data = withdraw(1);
    let accounts = [w.vault.info(), w.authority.info(), w.destination.info()];
    assert!(entry(&ID, &accounts, &data).is_err());
}

#[test]
fn cannot_withdraw_more_than_the_balance() {
    let mut w = world();
    w.authority.is_signer = true;
    let data = withdraw(VAULT_BALANCE + 1);
    let accounts = [w.vault.info(), w.authority.info(), w.destination.info()];
    assert!(entry(&ID, &accounts, &data).is_err());
}

#[test]
fn account_of_another_type_is_rejected() {
    let mut w = world();
    w.authority.is_signer = true;
    let mut not_a_vault = TestAccount::new(
        Pubkey::new_unique(),
        ID,
        VAULT_BALANCE,
        vec![0; w.vault.data.len()],
    );
    let data = withdraw(1);
    let accounts = [not_a_vault.info(), w.authority.info(), w.destination.info()];
    assert!(entry(&ID, &accounts, &data).is_err());
}
