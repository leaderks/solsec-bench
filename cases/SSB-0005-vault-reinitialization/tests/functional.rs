//! Behaviour every correct version must keep. Runs against the vulnerable and
//! the fixed program alike, and against any candidate patch.

mod common;

use common::{blank_vault, world, TestAccount, VAULT_BALANCE};
use solana_program::pubkey::Pubkey;
use ssb_0005::{initialize_data, initialized_vault, process_instruction, withdraw_data};

#[test]
fn fresh_vault_can_be_initialized() {
    let program_id = Pubkey::new_unique();
    let mut vault = blank_vault(program_id);
    let mut authority =
        TestAccount::new(Pubkey::new_unique(), Pubkey::default(), 0, vec![]).signer();
    let accounts = [vault.info(), authority.info()];
    process_instruction(&program_id, &accounts, &initialize_data()).unwrap();
    drop(accounts);
    assert_eq!(vault.data, initialized_vault(&authority.key));
}

#[test]
fn initialize_requires_the_authority_signature() {
    let program_id = Pubkey::new_unique();
    let mut vault = blank_vault(program_id);
    let mut authority = TestAccount::new(Pubkey::new_unique(), Pubkey::default(), 0, vec![]);
    let accounts = [vault.info(), authority.info()];
    assert!(process_instruction(&program_id, &accounts, &initialize_data()).is_err());
}

#[test]
fn authority_can_withdraw() {
    let mut w = world();
    w.authority.is_signer = true;
    let accounts = [w.vault.info(), w.authority.info(), w.destination.info()];
    process_instruction(&w.program_id, &accounts, &withdraw_data(9_000)).unwrap();
    drop(accounts);
    assert_eq!(w.vault.lamports, VAULT_BALANCE - 9_000);
    assert_eq!(w.destination.lamports, 9_000);
}

#[test]
fn other_signer_cannot_withdraw() {
    let mut w = world();
    let mut stranger =
        TestAccount::new(Pubkey::new_unique(), Pubkey::default(), 0, vec![]).signer();
    let accounts = [w.vault.info(), stranger.info(), w.destination.info()];
    assert!(process_instruction(&w.program_id, &accounts, &withdraw_data(1)).is_err());
}

#[test]
fn uninitialized_vault_cannot_be_withdrawn_from() {
    let program_id = Pubkey::new_unique();
    let mut vault = blank_vault(program_id);
    let mut anyone = TestAccount::new(Pubkey::new_unique(), Pubkey::default(), 0, vec![]).signer();
    let mut destination = TestAccount::new(Pubkey::new_unique(), Pubkey::default(), 0, vec![]);
    let accounts = [vault.info(), anyone.info(), destination.info()];
    assert!(process_instruction(&program_id, &accounts, &withdraw_data(1)).is_err());
}
