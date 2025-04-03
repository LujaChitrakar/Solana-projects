#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod votingdappp {
    use super::*;

  pub fn close(_ctx: Context<CloseVotingdappp>) -> Result<()> {
    Ok(())
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.votingdappp.count = ctx.accounts.votingdappp.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.votingdappp.count = ctx.accounts.votingdappp.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeVotingdappp>) -> Result<()> {
    Ok(())
  }

  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.votingdappp.count = value.clone();
    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitializeVotingdappp<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + Votingdappp::INIT_SPACE,
  payer = payer
  )]
  pub votingdappp: Account<'info, Votingdappp>,
  pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseVotingdappp<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub votingdappp: Account<'info, Votingdappp>,
}

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub votingdappp: Account<'info, Votingdappp>,
}

#[account]
#[derive(InitSpace)]
pub struct Votingdappp {
  count: u8,
}
