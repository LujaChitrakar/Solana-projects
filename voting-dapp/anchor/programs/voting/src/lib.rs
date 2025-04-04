#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

const ANCHOR_DISCRIMINATOR_SIZE: usize = 8; //anchor reserves 8 bytes

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(
        ctx: Context<InitializePoll>,
        poll_id: u64,
        description: String,
        poll_start: u64,
        poll_end: u64,
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll;

        poll.poll_id = poll_id;
        poll.description = description;
        poll.poll_start = poll_start;
        poll.poll_end = poll_end;
        poll.candidate_amount = 0;
        Ok(())
    }

    pub fn initialize_candidate(
        ctx:Context<InitializeCandidate>,
        candidate_name:String,
        _poll_id:u64
    )->Result<()>{
        let candidate=&mut ctx.accounts.candidate;
        let poll=&mut ctx.accounts.poll;
        poll.candidate_amount+=1;
        candidate.candidate_name=candidate_name;
        candidate.candidate_votes=0;
        Ok(())
    }

    pub fn vote(ctx: Context<InitializeVote>,_candidate_name:String,_poll_id:u64)->Result<()>{
        let candidate=&mut ctx.accounts.candidate;
        candidate.candidate_votes+=1;
        Ok(())
    }
}


/**-- INSTRUCTIONS --*/


#[derive(Accounts)]
#[instruction(poll_id:u64)] //used to pull paramter. here as u64
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init, //to make account automatically initialized.
        payer=signer,
        space=ANCHOR_DISCRIMINATOR_SIZE+Poll::INIT_SPACE,
        seeds=[poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    #[account()]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(candidate_name:String,poll_id:u64)] 
pub struct InitializeCandidate<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,

    #[account(
        mut,
        seeds=[poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll:Account<'info ,Poll>,

    #[account(
        init,
        payer=signer,
        space=ANCHOR_DISCRIMINATOR_SIZE+Candidate::INIT_SPACE,
        seeds=[poll_id.to_le_bytes().as_ref(),candidate_name.as_bytes()],
        bump
    )]
    pub candidate:Account<'info ,Candidate>,

    
    #[account()]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(candidate_name:String,poll_id:u64)]
pub struct InitializeVote<'info>{
    #[account()]
    signer:Signer<'info>,

    #[account(
        seeds=[poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll:Account<'info,Poll>,

    #[account(
        mut,
        seeds=[poll_id.to_le_bytes().as_ref(),candidate_name.as_bytes()],
        bump
    )]
    pub candidate:Account<'info,Candidate>,
    
}


/**-- STRUCTS --*/


#[account]
#[derive(InitSpace)] //automatically initialize required space
pub struct Poll {
    pub poll_id: u64,
    #[max_len(280)]
    //because in u64 max len is identified as 64 but in String max len is un identified
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64,
}

#[account]
#[derive(InitSpace)] //automatically initialize required space
pub struct Candidate{
    #[max_len(16)]
    pub candidate_name:String,
    candidate_votes:u64
}

#[account]
#[derive(InitSpace)]
pub struct Vote{

}
