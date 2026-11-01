#![allow(dead_code)]

use solana_program::{account_info::AccountInfo, pubkey::Pubkey};
use ssb_0003_account_type_confusion::{account_data, ACCOUNT_LEN, TAG_CONFIG};

/// Owned storage for one account, so tests can build `AccountInfo` views.
pub struct TestAccount {
    pub key: Pubkey,
    pub owner: Pubkey,
    pub lamports: u64,
    pub data: Vec<u8>,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl TestAccount {
    pub fn new(key: Pubkey, owner: Pubkey, lamports: u64, data: Vec<u8>) -> Self {
        Self {
            key,
            owner,
            lamports,
            data,
            is_signer: false,
            is_writable: true,
        }
    }

    pub fn signer(mut self) -> Self {
        self.is_signer = true;
        self
    }

    pub fn info(&mut self) -> AccountInfo<'_> {
        AccountInfo::new(
            &self.key,
            self.is_signer,
            self.is_writable,
            &mut self.lamports,
            &mut self.data,
            &self.owner,
            false,
        )
    }
}

pub const TREASURY_BALANCE: u64 = 50_000_000_000;

pub struct World {
    pub program_id: Pubkey,
    pub config: TestAccount,
    pub treasury: TestAccount,
    pub admin: TestAccount,
    pub destination: TestAccount,
}

/// A funded treasury governed by a program-owned config naming `admin`.
pub fn world() -> World {
    let program_id = Pubkey::new_unique();
    let admin = TestAccount::new(Pubkey::new_unique(), Pubkey::default(), 1_000_000, vec![]);
    let config = TestAccount::new(
        Pubkey::new_unique(),
        program_id,
        1_000_000,
        account_data(TAG_CONFIG, &admin.key),
    );
    let treasury = TestAccount::new(Pubkey::new_unique(), program_id, TREASURY_BALANCE, vec![]);
    let destination = TestAccount::new(Pubkey::new_unique(), Pubkey::default(), 0, vec![]);
    World {
        program_id,
        config,
        treasury,
        admin,
        destination,
    }
}

/// A fresh, zeroed account assigned to the program, as a client creates it
/// with the system program before calling `Register`.
pub fn blank_profile(program_id: Pubkey) -> TestAccount {
    TestAccount::new(
        Pubkey::new_unique(),
        program_id,
        1_000_000,
        vec![0; ACCOUNT_LEN],
    )
}
