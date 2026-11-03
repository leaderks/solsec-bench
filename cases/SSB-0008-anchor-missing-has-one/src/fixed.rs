use anchor_lang::prelude::*;

declare_id!("9hTP2TvpwmgXPP13zimfTBuzC5KAbn38Yp3Pry8QtXqN");

#[program]
pub mod vault {
    use super::*;

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault = ctx.accounts.vault.to_account_info();
        let destination = ctx.accounts.destination.to_account_info();
        let remaining = vault
            .lamports()
            .checked_sub(amount)
            .ok_or(VaultError::InsufficientFunds)?;
        let credited = destination
            .lamports()
            .checked_add(amount)
            .ok_or(VaultError::Overflow)?;
        **vault.try_borrow_mut_lamports()? = remaining;
        **destination.try_borrow_mut_lamports()? = credited;
        Ok(())
    }
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, has_one = authority)]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
    /// CHECK: any account may receive lamports.
    #[account(mut)]
    pub destination: UncheckedAccount<'info>,
}

#[error_code]
pub enum VaultError {
    #[msg("vault balance is too low")]
    InsufficientFunds,
    #[msg("arithmetic overflow")]
    Overflow,
}
