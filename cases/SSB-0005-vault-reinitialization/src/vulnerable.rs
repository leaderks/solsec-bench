use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{IX_INITIALIZE, IX_WITHDRAW, STATE_INITIALIZED, VAULT_LEN};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (tag, rest) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    match *tag {
        IX_INITIALIZE if rest.is_empty() => initialize(program_id, accounts),
        IX_WITHDRAW => {
            let amount = rest
                .try_into()
                .map(u64::from_le_bytes)
                .map_err(|_| ProgramError::InvalidInstructionData)?;
            withdraw(program_id, accounts, amount)
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

/// Accounts: `[vault (writable, program-owned), authority (signer)]`.
fn initialize(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let iter = &mut accounts.iter();
    let vault = next_account_info(iter)?;
    let authority = next_account_info(iter)?;

    if vault.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    let mut data = vault.try_borrow_mut_data()?;
    if data.len() != VAULT_LEN {
        return Err(ProgramError::InvalidAccountData);
    }
    data[0] = STATE_INITIALIZED;
    data[1..].copy_from_slice(authority.key.as_ref());
    Ok(())
}

/// Accounts: `[vault (writable), authority (signer), destination (writable)]`.
fn withdraw(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let iter = &mut accounts.iter();
    let vault = next_account_info(iter)?;
    let authority = next_account_info(iter)?;
    let destination = next_account_info(iter)?;

    if vault.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    {
        let data = vault.try_borrow_data()?;
        if data.len() != VAULT_LEN || data[0] != STATE_INITIALIZED {
            return Err(ProgramError::UninitializedAccount);
        }
        if data[1..] != authority.key.as_ref()[..] {
            msg!("signer is not the vault authority");
            return Err(ProgramError::InvalidAccountData);
        }
    }

    let remaining = vault
        .lamports()
        .checked_sub(amount)
        .ok_or(ProgramError::InsufficientFunds)?;
    **vault.try_borrow_mut_lamports()? = remaining;
    let credited = destination
        .lamports()
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    **destination.try_borrow_mut_lamports()? = credited;
    Ok(())
}
