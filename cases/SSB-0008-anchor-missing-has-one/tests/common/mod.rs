#![allow(dead_code)]

use anchor_lang::prelude::{AccountInfo, Pubkey};
use anchor_lang::AccountSerialize;
use ssb_0008::{Vault, ID};

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

pub const VAULT_BALANCE: u64 = 40_000_000_000;

/// Serialized `Vault` account data, discriminator included.
pub fn vault_data(authority: Pubkey) -> Vec<u8> {
    let mut data = Vec::new();
    Vault { authority }.try_serialize(&mut data).unwrap();
    data
}

pub struct World {
    pub authority: TestAccount,
    pub vault: TestAccount,
    pub destination: TestAccount,
}

/// A funded vault whose recorded authority is `authority`.
pub fn world() -> World {
    let authority = TestAccount::new(Pubkey::new_unique(), Pubkey::default(), 1_000_000, vec![]);
    let vault = TestAccount::new(
        Pubkey::new_unique(),
        ID,
        VAULT_BALANCE,
        vault_data(authority.key),
    );
    let destination = TestAccount::new(Pubkey::new_unique(), Pubkey::default(), 0, vec![]);
    World {
        authority,
        vault,
        destination,
    }
}
