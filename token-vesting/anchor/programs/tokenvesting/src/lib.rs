#![allow(clippy::result_large_err)]
pub mod instructions;

use anchor_lang::prelude::*;
use instructions::create_vesting::*;
declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod tokenvesting {

    use anchor_lang::accounts::signer;
    use anchor_spl::token::{self, TransferChecked};

    use super::*;

    pub fn create_vesting_account(
        ctx: Context<CreateVestingAccount>,
        company_name: String,
    ) -> Result<()> {
        *ctx.accounts.vesting_account = VestingAccount {
            owner: ctx.accounts.signer.key(),
            mint: ctx.accounts.mint.key(),
            treasury_token_account: ctx.accounts.treasury_token_account.key(),
            company_name: company_name,
            treasurey_bump: ctx.bumps.treasury_token_account,
            bump: ctx.bumps.vesting_account,
        };
        Ok(())
    }

    pub fn create_employee_account(
        ctx: Context<CreateEmployeeAccount>,
        start_time: i64,
        end_time: i64,
        cliff_time: i64,
        total_amount: u64,
    ) -> Result<()> {
        *ctx.accounts.employee_account = EmployeeAccount {
            beneficiary: ctx.accounts.owner.key(),
            start_time: start_time,
            end_time: end_time,
            cliff_time: cliff_time,
            vesting_account: ctx.accounts.vesting_account.key(),
            total_amount: total_amount,
            total_withdraw: 0,
            bump: ctx.bumps.employee_account,
        };
        Ok(())
    }

    pub fn claim_tokens(ctx: Context<ClaimTokens>, _company_name: String) -> Result<()> {
        let employee_account = &mut ctx.accounts.employee_account;

        let current_time = Clock::get()?.unix_timestamp;
        if current_time < employee_account.cliff_time {
            return Err(ErrorCode::ClaimNotAvailableYet.into());
        }

        let time_since_start = current_time.saturating_sub(employee_account.start_time);
        let total_vesting_time = employee_account
            .end_time
            .saturating_sub(employee_account.start_time);

        if total_vesting_time == 0 {
            return Err(ErrorCode::InvalidVestingPeriod.into());
        }

        let vested_amount = if current_time >= employee_account.end_time {
            employee_account.total_amount
        } else {
            match employee_account
                .total_amount
                .checked_mul(time_since_start as u64)
            {
                Some(product) => product / total_vesting_time as u64,
                None => return Err(ErrorCode::CalculationOverflow.into()),
            }
        };

        let claimable_amount = vested_amount.saturating_sub(employee_account.total_withdraw);
        if claimable_amount == 0 {
            return Err(ErrorCode::NothingToClaim.into());
        }

        let transfer_cpi_account = TransferChecked {
            from: ctx.accounts.treasury_token_account.to_account_info(),
            to: ctx.accounts.employee_token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            authority: ctx.accounts.treasury_token_account.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"vesting_treasury",
            ctx.accounts.vesting_account.company_name.as_ref(),
            &[ctx.accounts.vesting_account.treasurey_bump],
        ]];

        let cpi_context =
            CpiContext::new(cpi_program, transfer_cpi_account).with_signer(signer_seeds);
        let decimals = ctx.accounts.mint.decimals;

        token::transfer_checked(cpi_context, claimable_amount as u64, decimals)?;

        employee_account.total_withdraw += claimable_amount;
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Claim Not Available Yet")]
    ClaimNotAvailableYet,
    #[msg("Invalud vesting period")]
    InvalidVestingPeriod,
    #[msg("Overflow in calculation")]
    CalculationOverflow,
    #[msg("There is nothing to claim")]
    NothingToClaim,
}
