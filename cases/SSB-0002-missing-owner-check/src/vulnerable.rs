use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{CONFIG_LEN, IX_PAYOUT};

/// Accounts: `[config, treasury (writable), admin (signer), destination (writable)]`.
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (tag, rest) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    if *tag != IX_PAYOUT {
        return Err(ProgramError::InvalidInstructionData);
    }
    let amount = rest
        .try_into()
        .map(u64::from_le_bytes)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    let iter = &mut accounts.iter();
    let config = next_account_info(iter)?;
    let treasury = next_account_info(iter)?;
    let admin = next_account_info(iter)?;
    let destination = next_account_info(iter)?;

    if treasury.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    let configured_admin = {
        let data = config.try_borrow_data()?;
        if data.len() != CONFIG_LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Pubkey::try_from(&data[..]).map_err(|_| ProgramError::InvalidAccountData)?
    };
    if !admin.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if *admin.key != configured_admin {
        msg!("signer is not the configured admin");
        return Err(ProgramError::InvalidAccountData);
    }

    let remaining = treasury
        .lamports()
        .checked_sub(amount)
        .ok_or(ProgramError::InsufficientFunds)?;
    **treasury.try_borrow_mut_lamports()? = remaining;
    let credited = destination
        .lamports()
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    **destination.try_borrow_mut_lamports()? = credited;
    Ok(())
}
