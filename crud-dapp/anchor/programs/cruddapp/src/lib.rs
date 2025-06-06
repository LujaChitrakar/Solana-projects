#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("FQbq9Lvg86BuKAdkvx6HeN87DWhHijdBDaisd1YAhfi9");

#[program]
pub mod cruddapp{
  use super::*;

  pub fn create_journal_entry(ctx:Context<CreateEntry>,title:String,message:String)->Result<()>{
    let journal_entry=&mut ctx.accounts.journal_entry;
    journal_entry.owner=*ctx.accounts.owner.key;
    journal_entry.title=title;
    journal_entry.message=message;
    Ok(())
  }

  pub fn update_entry(ctx:Context<UpdateEntry>,_title:String,message:String)->Result<()>{
    let journal_entry=&mut ctx.accounts.journal_entry;
    journal_entry.message=message;
    Ok(())
  }

  pub fn delete_entry(_ctx:Context<DeleteEntry>,_title:String)->Result<()>{
    Ok(())
  }
}

/** INSTRUCTIONS/FUNCTIONS */

#[derive(Accounts)]
#[instruction(title:String)]
pub struct CreateEntry<'info>{
  #[account(mut)]
   pub owner:Signer<'info>,

  #[account(
    init,
    payer=owner,
    space=8+JournalEntryState::INIT_SPACE,
    seeds=[title.as_bytes(),owner.key().as_ref()],
    bump,
  )]
  pub journal_entry:Account<'info,JournalEntryState>,

  #[account()]
  pub system_program:Program<'info,System>,
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct UpdateEntry<'info>{
  #[account(mut)]
  pub owner:Signer<'info>,

  #[account(
    mut,
    seeds=[title.as_bytes(),owner.key().as_ref()],
    bump,
    realloc=8+JournalEntryState::INIT_SPACE, //re allocating space
    realloc::payer=owner,
    realloc::zero=true,
  )]
  pub journal_entry:Account<'info,JournalEntryState>,

  #[account()]
  pub system_program:Program<'info,System>,

}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct DeleteEntry<'info>{
  #[account(mut)]
  pub owner:Signer<'info>,

  #[account(
    mut,
    seeds=[title.as_bytes(),owner.key().as_ref()],
    bump,
    close=owner,
  )]
  pub journal_entry:Account<'info,JournalEntryState>,

  #[account()]
  pub system_program:Program<'info,System>,
}
/** STATES */

#[account]
#[derive(InitSpace)]
pub struct JournalEntryState{
  pub owner:Pubkey,
  #[max_len(50)]
  pub title:String,
  #[max_len(1000)]
  pub message:String
} 