#![allow(clippy::result_large_err)]
pub mod instructions;
pub use instructions::*;

use anchor_lang::prelude::*;
declare_id!("4VNuGptng1b6vwb8fAMsy5Y5Kg7rKsaZT6bTXL9rrK8U");

#[program]
pub mod tokenlottery {

    use super::*;
    pub fn intialize_config(ctx:Context<InitializeConfig>,start_time:u64,end_time:u64,ticket_price:u64)->Result<()>{
      ctx.accounts.token_lottery.bump=ctx.bumps.token_lottery;
      ctx.accounts.token_lottery.start_time=start_time;
      ctx.accounts.token_lottery.end_time=end_time;
      ctx.accounts.token_lottery.ticket_price=ticket_price;
      ctx.accounts.token_lottery.authority=*ctx.accounts.payer.key;
      ctx.accounts.token_lottery.lottery_pot_amount=0;
      ctx.accounts.token_lottery.total_tickets=0;
      ctx.accounts.token_lottery.randomness_account=Pubkey::default();
      ctx.accounts.token_lottery.winner_chosen=false;
      Ok(())
    }

}
