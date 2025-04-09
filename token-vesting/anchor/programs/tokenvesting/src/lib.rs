#![allow(clippy::result_large_err)]
pub mod instructions;

use anchor_lang::prelude::*;
use instructions::create_vesting::*;
declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod tokenvesting {
    use super::*;

  pub fn create_vesting_account(ctx:Context<CreateVestingAccount>,company_name:String)->Result<()>{
    *ctx.accounts.vesting_account=VestingAccount{
      owner:ctx.accounts.signer.key(),
      mint:ctx.accounts.mint.key(),
      treasury_token_account:ctx.accounts.treasury_token_account.key(),
      company_name:company_name,
      treasurey_bump:ctx.bumps.treasury_token_account,
      bump:ctx.bumps.vesting_account,
    };
    Ok(())
  }

  pub fn create_employee_account(ctx:Context<CreateEmployeeAccount>,start_time:i64,end_time:i64,cliff_time:i64,total_amount:u64)->Result<()>{
    *ctx.accounts.employee_account=EmployeeAccount{
      beneficiary:ctx.accounts.owner.key(),
      start_time:start_time,
      end_time:end_time,
      cliff_time:cliff_time,
      vesting_account:ctx.accounts.vesting_account.key(),
      total_amount:total_amount,
      total_withdraw:0,
      bump:ctx.bumps.employee_account,
    };
    Ok(())
  }
}

