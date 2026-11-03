#![allow(dead_code)]

use solana_program::{account_info::AccountInfo, pubkey::Pubkey};
use ssb_0007::ledger;

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

pub const ALICE_BALANCE: u64 = 1_000;
pub const BOB_BALANCE: u64 = 500;

pub struct World {
    pub program_id: Pubkey,
    pub alice: TestAccount,
    pub alice_ledger: TestAccount,
    pub bob_ledger: TestAccount,
}

/// Two funded ledgers, one owned by `alice` and one by another user.
pub fn world() -> World {
    let program_id = Pubkey::new_unique();
    let alice = TestAccount::new(Pubkey::new_unique(), Pubkey::default(), 1_000_000, vec![]);
    let alice_ledger = TestAccount::new(
        Pubkey::new_unique(),
        program_id,
        1_000_000,
        ledger(&alice.key, ALICE_BALANCE),
    );
    let bob_ledger = TestAccount::new(
        Pubkey::new_unique(),
        program_id,
        1_000_000,
        ledger(&Pubkey::new_unique(), BOB_BALANCE),
    );
    World {
        program_id,
        alice,
        alice_ledger,
        bob_ledger,
    }
}
