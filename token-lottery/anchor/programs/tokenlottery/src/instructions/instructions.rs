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

    pub rent: Sysvar<'info, Rent>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds=[b"token_lottery".as_ref()],
        bump=token_lottery.bump,
    )]
    pub token_lottery: Account<'info, TokenLottery>,

    #[account(
        mut,
        seeds=[b"collection_mint".as_ref()],
        bump
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    /// CHECK: This account is validated by the token metadata program during instruction execution.
    #[account(
        mut,
        seeds=[b"metadata",token_metadata_program.key().as_ref(),ticket_mint.key().as_ref()],
        bump,
        seeds::program=token_metadata_program.key()
    )]
    pub ticket_metadata: UncheckedAccount<'info>,

    /// CHECK: This account is validated by the token metadata program during instruction execution.
    #[account(
        mut,
        seeds=[
       b"metadata",
            token_metadata_program.key().as_ref(),
            ticket_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program=token_metadata_program.key(),
    )]
    pub ticket_master_edition: UncheckedAccount<'info>,

    /// CHECK: This account is validated by the token metadata program during instruction execution.
    #[account(
        mut,
        seeds=[b"metadata",token_metadata_program.key().as_ref(),collection_mint.key().as_ref()],
        bump,
        seeds::program=token_metadata_program.key()
    )]
    pub collection_metadata: UncheckedAccount<'info>,

    /// CHECK: This account is validated by the token metadata program during instruction execution.
    #[account(
        mut,
        seeds=[
       b"metadata",
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program=token_metadata_program.key(),
    )]
    pub collection_master_edition: UncheckedAccount<'info>,

    #[account(
        init,
        payer=payer,
        mint::decimals=0,
        mint::authority=collection_mint,
        mint::freeze_authority=collection_mint,
        seeds=[token_lottery.total_tickets.to_le_bytes().as_ref()],
        bump
    )]
    pub ticket_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer=payer,
        associated_token::mint=ticket_mint,
        associated_token::authority=payer,
        associated_token::token_program=token_program,
    )]
    pub destination: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
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
