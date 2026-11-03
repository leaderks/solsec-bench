#![allow(dead_code)]

use solana_program::{account_info::AccountInfo, pubkey::Pubkey};
use ssb_0006::valid_ticket;

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

pub const TREASURY_BALANCE: u64 = 10_000_000_000;
pub const TICKET_RENT: u64 = 1_200_000;

pub struct World {
    pub program_id: Pubkey,
    pub holder: TestAccount,
    pub ticket: TestAccount,
    pub treasury: TestAccount,
    pub destination: TestAccount,
}

/// A funded treasury and one valid ticket held by `holder`.
pub fn world() -> World {
    let program_id = Pubkey::new_unique();
    let holder = TestAccount::new(Pubkey::new_unique(), Pubkey::default(), 1_000_000, vec![]);
    let ticket = TestAccount::new(
        Pubkey::new_unique(),
        program_id,
        TICKET_RENT,
        valid_ticket(&holder.key),
    );
    let treasury = TestAccount::new(Pubkey::new_unique(), program_id, TREASURY_BALANCE, vec![]);
    let destination = TestAccount::new(Pubkey::new_unique(), Pubkey::default(), 0, vec![]);
    World {
        program_id,
        holder,
        ticket,
        treasury,
        destination,
    }
}
