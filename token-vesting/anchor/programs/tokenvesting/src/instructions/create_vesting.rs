use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};



/*-- For employer  --*/
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

/*-- For Employee --*/
#[derive(Accounts)]
pub struct CreateEmployeeAccount<'info>{
    #[account(mut)]
    pub owner:Signer<'info>,
    
    #[account(has_one=owner)]
    pub vesting_account:Account<'info,VestingAccount>,

    #[account(
        init,
        payer=owner,
        space=8+EmployeeAccount::INIT_SPACE,
        seeds=[b"employee_vesting",beneficiary.key().as_ref(),vesting_account.key().as_ref()],
        bump
    )]
    pub employee_account:Account<'info,EmployeeAccount>,

    pub beneficiary:SystemAccount<'info>,
    pub system_program:Program<'info,System>

}

#[account]
#[derive(InitSpace)]
pub struct EmployeeAccount{
    pub beneficiary:Pubkey,
    pub start_time:i64,
    pub end_time:i64,
    pub cliff_time:i64,
    pub vesting_account:Pubkey,
    pub total_amount:u64,
    pub total_withdraw:u64,
    pub bump:u8
}