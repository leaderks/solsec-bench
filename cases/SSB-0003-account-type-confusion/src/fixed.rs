use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{ACCOUNT_LEN, IX_PAYOUT, IX_REGISTER, TAG_CONFIG, TAG_PROFILE, TAG_UNINITIALIZED};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (tag, rest) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    match *tag {
        IX_PAYOUT => {
            let amount = rest
                .try_into()
                .map(u64::from_le_bytes)
                .map_err(|_| ProgramError::InvalidInstructionData)?;
            payout(program_id, accounts, amount)
        }
        IX_REGISTER if rest.is_empty() => register(program_id, accounts),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

/// Accounts: `[profile (writable, program-owned, uninitialized), user (signer)]`.
fn register(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let iter = &mut accounts.iter();
    let profile = next_account_info(iter)?;
    let user = next_account_info(iter)?;

    if profile.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    if !user.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    let mut data = profile.try_borrow_mut_data()?;
    if data.len() != ACCOUNT_LEN {
        return Err(ProgramError::InvalidAccountData);
    }
    if data[0] != TAG_UNINITIALIZED {
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    data[0] = TAG_PROFILE;
    data[1..].copy_from_slice(user.key.as_ref());
    Ok(())
}

/// Accounts: `[config, treasury (writable), admin (signer), destination (writable)]`.
fn payout(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let iter = &mut accounts.iter();
    let config = next_account_info(iter)?;
    let treasury = next_account_info(iter)?;
    let admin = next_account_info(iter)?;
    let destination = next_account_info(iter)?;

    if treasury.owner != program_id || config.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    let configured_admin = {
        let data = config.try_borrow_data()?;
        if data.len() != ACCOUNT_LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        if data[0] != TAG_CONFIG {
            msg!("config account is not a config");
            return Err(ProgramError::InvalidAccountData);
        }
        Pubkey::try_from(&data[1..]).map_err(|_| ProgramError::InvalidAccountData)?
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
