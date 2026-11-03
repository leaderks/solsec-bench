use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{IX_TRANSFER, LEDGER_LEN};

fn read_ledger(account: &AccountInfo) -> Result<(Pubkey, u64), ProgramError> {
    let data = account.try_borrow_data()?;
    if data.len() != LEDGER_LEN {
        return Err(ProgramError::InvalidAccountData);
    }
    let owner = Pubkey::try_from(&data[..32]).map_err(|_| ProgramError::InvalidAccountData)?;
    let balance = u64::from_le_bytes(data[32..].try_into().unwrap());
    Ok((owner, balance))
}

fn write_balance(account: &AccountInfo, balance: u64) -> ProgramResult {
    account.try_borrow_mut_data()?[32..].copy_from_slice(&balance.to_le_bytes());
    Ok(())
}

/// Accounts: `[from (writable), to (writable), owner of from (signer)]`.
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (tag, rest) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    if *tag != IX_TRANSFER {
        return Err(ProgramError::InvalidInstructionData);
    }
    let amount = rest
        .try_into()
        .map(u64::from_le_bytes)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    let iter = &mut accounts.iter();
    let from = next_account_info(iter)?;
    let to = next_account_info(iter)?;
    let owner = next_account_info(iter)?;

    if from.owner != program_id || to.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    if !owner.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if from.key == to.key {
        msg!("source and destination ledgers must differ");
        return Err(ProgramError::InvalidArgument);
    }
    let (from_owner, from_balance) = read_ledger(from)?;
    let (_, to_balance) = read_ledger(to)?;
    if from_owner != *owner.key {
        msg!("signer does not own the source ledger");
        return Err(ProgramError::InvalidAccountData);
    }

    let new_from = from_balance
        .checked_sub(amount)
        .ok_or(ProgramError::InsufficientFunds)?;
    let new_to = to_balance
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    write_balance(from, new_from)?;
    write_balance(to, new_to)?;
    Ok(())
}
