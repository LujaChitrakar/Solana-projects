use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeConfig<'info>{
    #[account(mut)]
    pub payer:Signer<'info>,

    #[account(
        init,
        payer=payer,
        space=8+TokenLottery::INIT_SPACE,
        seeds=[b"token_lottery"],
        bump
    )]
    pub token_lottery:Account<'info,TokenLottery>,

    pub system_program:Program<'info,System>
}

#[account]
#[derive(InitSpace)]
pub struct TokenLottery{
    pub winner:u64,
    pub winner_chosen:bool,
    pub start_time:u64,
    pub end_time:u64,
    pub lottery_pot_amount:u64,
    pub total_tickets:u64,
    pub ticket_price:u64,
    pub authority:Pubkey,
    pub randomness_account:Pubkey,
    pub bump:u8,
}