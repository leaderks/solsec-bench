use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{IX_WITHDRAW, VAULT_LEN};

/// Accounts: `[vault (writable), authority, destination (writable)]`.
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (tag, rest) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    if *tag != IX_WITHDRAW {
        return Err(ProgramError::InvalidInstructionData);
    }
    let amount = rest
        .try_into()
        .map(u64::from_le_bytes)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    let iter = &mut accounts.iter();
    let vault = next_account_info(iter)?;
    let authority = next_account_info(iter)?;
    let destination = next_account_info(iter)?;

    if vault.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    let stored = {
        let data = vault.try_borrow_data()?;
        if data.len() != VAULT_LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Pubkey::try_from(&data[..]).map_err(|_| ProgramError::InvalidAccountData)?
    };
    if !authority.is_signer {
        msg!("authority must sign");
        return Err(ProgramError::MissingRequiredSignature);
    }
    if *authority.key != stored {
        msg!("authority does not match vault");
        return Err(ProgramError::InvalidAccountData);
    }

    let vault_lamports = vault.lamports();
    let remaining = vault_lamports
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
