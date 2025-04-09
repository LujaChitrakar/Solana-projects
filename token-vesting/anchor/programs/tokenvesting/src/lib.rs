#![allow(clippy::result_large_err)]
pub mod instructions;

use anchor_lang::prelude::*;
use instructions::create_vesting::*;
declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod tokenvesting {
    use super::*;

  pub fn create_vesting_account(ctx:Context<CreateVestingAccount
    >,company_name:String)->Result<()>{
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
}

