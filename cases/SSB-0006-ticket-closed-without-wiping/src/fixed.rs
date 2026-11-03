use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{IX_REDEEM, PRIZE, STATE_VALID, TICKET_LEN};

/// Accounts: `[ticket (writable), holder (signer), treasury (writable), destination (writable)]`.
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if data != [IX_REDEEM] {
        return Err(ProgramError::InvalidInstructionData);
    }

    let iter = &mut accounts.iter();
    let ticket = next_account_info(iter)?;
    let holder = next_account_info(iter)?;
    let treasury = next_account_info(iter)?;
    let destination = next_account_info(iter)?;

    if ticket.owner != program_id || treasury.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    if !holder.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    {
        let data = ticket.try_borrow_data()?;
        if data.len() != TICKET_LEN || data[0] != STATE_VALID {
            msg!("ticket is not valid");
            return Err(ProgramError::InvalidAccountData);
        }
        if data[1..] != holder.key.as_ref()[..] {
            msg!("signer does not hold this ticket");
            return Err(ProgramError::InvalidAccountData);
        }
    }

    // Pay the prize.
    let remaining = treasury
        .lamports()
        .checked_sub(PRIZE)
        .ok_or(ProgramError::InsufficientFunds)?;
    **treasury.try_borrow_mut_lamports()? = remaining;

    // Close the ticket and refund its rent along with the prize.
    let refund = ticket.lamports();
    **ticket.try_borrow_mut_lamports()? = 0;
    ticket.try_borrow_mut_data()?.fill(0);
    let credited = destination
        .lamports()
        .checked_add(PRIZE)
        .and_then(|l| l.checked_add(refund))
        .ok_or(ProgramError::ArithmeticOverflow)?;
    **destination.try_borrow_mut_lamports()? = credited;
    Ok(())
}
