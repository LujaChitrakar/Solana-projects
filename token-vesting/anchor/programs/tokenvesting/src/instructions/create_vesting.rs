use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};


#[derive(Accounts)]
#[instruction(company_name:String)]
pub struct CreateVestingAccount<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,

    #[account(
        init,
        payer=signer,
        space=8+VestingAccount::INIT_SPACE,
        seeds=[company_name.as_ref()],
        bump,
    )]
    pub vesting_account:Account<'info,VestingAccount>,

    #[account()]
    pub mint:Account<'info,Mint>,

    #[account(
        init,
        payer=signer,
        token::mint=mint,
        token::authority=treasury_token_account,
        seeds=[b"vesting_treasury",company_name.as_bytes()],
        bump
    )]
    pub treasury_token_account:Account<'info,TokenAccount>,

    pub token_program:Program<'info,Token>,

    pub system_program:Program<'info,System>
}

#[account]
#[derive(InitSpace)]
pub struct VestingAccount{
    pub owner:Pubkey,
    pub mint:Pubkey,
    pub treasury_token_account:Pubkey,
    #[max_len(50)]
    pub company_name:String,
    pub treasurey_bump:u8,
    pub bump:u8
}
