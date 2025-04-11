use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

/*----------- INITIALIZE CONFIG -----------*/
#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer=payer,
        space=8+TokenLottery::INIT_SPACE,
        seeds=[b"token_lottery"],
        bump
    )]
    pub token_lottery: Account<'info, TokenLottery>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct TokenLottery {
    pub winner: u64,
    pub winner_chosen: bool,
    pub start_time: u64,
    pub end_time: u64,
    pub lottery_pot_amount: u64,
    pub total_tickets: u64,
    pub ticket_price: u64,
    pub authority: Pubkey,
    pub randomness_account: Pubkey,
    pub bump: u8,
}

/*----------- INITIALIZE LOTTERY -----------*/

#[derive(Accounts)]
pub struct InitializeLottery<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer=payer,
        mint::decimals=0,
        mint::authority=collection_mint,
        mint::freeze_authority=collection_mint,
        seeds=[b"collection_mint".as_ref()],
        bump
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer=payer,
        token::mint=collection_mint,
        token::authority=collection_token_account,
        seeds=[b"collection_associated_token".as_ref()],
        bump
    )]
    pub collection_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: This account is validated by the token metadata program during instruction execution.
    #[account(
        mut,
        seeds=[
            b"metabata",
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref()
            ],
        bump,
        seeds::program=token_metadata_program.key())]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: This account is validated by the token metadata program during instruction execution.
    #[account(
        mut,
        seeds=[
            b"metabata",
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref(),
            b"edition".as_ref()],
        bump,
        seeds::program=token_metadata_program.key())]
    pub master_edition: UncheckedAccount<'info>,

    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
