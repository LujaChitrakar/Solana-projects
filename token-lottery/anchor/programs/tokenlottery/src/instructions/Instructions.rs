use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeConfig<'info>{
    #[account(mut)]
    pub payer:Signer<'info>,

    #[account(
        init,
        payer=payer,
        space=8,
        seeds=[b"token_lottery"],
        bump
    )]
    pub token_lottery:Account<'info,TokenLottery>,

    pub system_program:Program<'info,System>
}

#[account]
pub struct TokenLottery{
    
}