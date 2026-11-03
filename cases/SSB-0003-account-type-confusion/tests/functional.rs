//! Behaviour every correct version must keep. Runs against the vulnerable and
//! the fixed program alike, and against any candidate patch.

mod common;

use common::{blank_profile, world, TestAccount, TREASURY_BALANCE};
use solana_program::pubkey::Pubkey;
use ssb_0003::{
    account_data, payout_data, process_instruction, register_data, TAG_CONFIG, TAG_PROFILE,
};

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
fn config_owned_by_another_program_is_rejected() {
    let mut w = world();
    w.admin.is_signer = true;
    w.config.owner = Pubkey::new_unique();
    let accounts = [
        w.config.info(),
        w.treasury.info(),
        w.admin.info(),
        w.destination.info(),
    ];
    assert!(process_instruction(&w.program_id, &accounts, &payout_data(1)).is_err());
}

#[test]
fn user_can_register_a_profile() {
    let program_id = Pubkey::new_unique();
    let mut profile = blank_profile(program_id);
    let mut user = TestAccount::new(Pubkey::new_unique(), Pubkey::default(), 0, vec![]).signer();
    let accounts = [profile.info(), user.info()];
    process_instruction(&program_id, &accounts, &register_data()).unwrap();
    drop(accounts);
    assert_eq!(profile.data, account_data(TAG_PROFILE, &user.key));
}

#[test]
fn register_requires_the_user_signature() {
    let program_id = Pubkey::new_unique();
    let mut profile = blank_profile(program_id);
    let mut user = TestAccount::new(Pubkey::new_unique(), Pubkey::default(), 0, vec![]);
    let accounts = [profile.info(), user.info()];
    assert!(process_instruction(&program_id, &accounts, &register_data()).is_err());
}

#[test]
fn initialized_accounts_cannot_be_registered_again() {
    let mut w = world();
    let mut user = TestAccount::new(Pubkey::new_unique(), Pubkey::default(), 0, vec![]).signer();
    let program_id = w.program_id;
    let accounts = [w.config.info(), user.info()];
    assert!(process_instruction(&program_id, &accounts, &register_data()).is_err());
    drop(accounts);
    assert_eq!(w.config.data, account_data(TAG_CONFIG, &w.admin.key));
}
