use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

/*-- For employer  --*/
#[derive(Accounts)]
#[instruction(company_name:String)]
pub struct CreateVestingAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer=signer,
        space=8+VestingAccount::INIT_SPACE,
        seeds=[company_name.as_ref()],
        bump,
    )]
    pub vesting_account: Account<'info, VestingAccount>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer=signer,
        token::mint=mint,
        token::authority=treasury_token_account,
        seeds=[b"vesting_treasury",company_name.as_bytes()],
        bump
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct VestingAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub treasury_token_account: Pubkey,
    #[max_len(50)]
    pub company_name: String,
    pub treasurey_bump: u8,
    pub bump: u8,
}

/*-- For Employee --*/

#[derive(Accounts)]
pub struct CreateEmployeeAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(has_one=owner)]
    pub vesting_account: Account<'info, VestingAccount>,

    #[account(
        init,
        payer=owner,
        space=8+EmployeeAccount::INIT_SPACE,
        seeds=[b"employee_vesting",beneficiary.key().as_ref(),vesting_account.key().as_ref()],
        bump
    )]
    pub employee_account: Account<'info, EmployeeAccount>,

    pub beneficiary: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct EmployeeAccount {
    pub beneficiary: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
    pub cliff_time: i64,
    pub vesting_account: Pubkey,
    pub total_amount: u64,
    pub total_withdraw: u64,
    pub bump: u8,
}

/*-- Claim Tokens --*/

#[derive(Accounts)]
#[instruction(company_name:String)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub beneficiary: Signer<'info>,

    #[account(
        mut,
        seeds=[b"employee_vesting",beneficiary.key().as_ref(),vesting_account.key().as_ref()],
        has_one=beneficiary,
        has_one=vesting_account,
        bump=employee_account.bump
    )]
    pub employee_account: Account<'info, EmployeeAccount>,

    #[account(
        mut,
        seeds=[company_name.as_ref()],
        has_one=treasury_token_account,
        has_one=mint,
        bump=vesting_account.bump,
    )]
    pub vesting_account: Account<'info, VestingAccount>,

    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer=beneficiary,
        associated_token::mint=mint,
        associated_token::authority=beneficiary,
        associated_token::token_program=token_program,
    )]
    pub employee_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
