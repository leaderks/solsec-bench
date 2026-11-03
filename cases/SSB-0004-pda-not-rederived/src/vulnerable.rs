use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::IX_WITHDRAW;

/// Accounts: `[vault (writable), owner (signer), destination (writable)]`.
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
    let owner = next_account_info(iter)?;
    let destination = next_account_info(iter)?;

    if vault.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    if !owner.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
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
