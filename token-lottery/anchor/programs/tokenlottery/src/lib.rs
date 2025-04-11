#![allow(clippy::result_large_err)]
use anchor_spl::token_interface::{mint_to, MintTo};
use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;

declare_id!("4VNuGptng1b6vwb8fAMsy5Y5Kg7rKsaZT6bTXL9rrK8U");

#[program]
pub mod tokenlottery {

    use anchor_spl::metadata::create_metadata_accounts_v3;

    use super::*;

    pub fn intialize_config(
        ctx: Context<InitializeConfig>,
        start_time: u64,
        end_time: u64,
        ticket_price: u64,
    ) -> Result<()> {
        ctx.accounts.token_lottery.bump = ctx.bumps.token_lottery;
        ctx.accounts.token_lottery.start_time = start_time;
        ctx.accounts.token_lottery.end_time = end_time;
        ctx.accounts.token_lottery.ticket_price = ticket_price;
        ctx.accounts.token_lottery.authority = *ctx.accounts.payer.key;
        ctx.accounts.token_lottery.lottery_pot_amount = 0;
        ctx.accounts.token_lottery.total_tickets = 0;
        ctx.accounts.token_lottery.randomness_account = Pubkey::default();
        ctx.accounts.token_lottery.winner_chosen = false;
        Ok(())
    }

    pub fn initialize_lottery(ctx: Context<InitializeLottery>) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] =
            &[&[b"collection_mint".as_ref(), &[ctx.bumps.collection_mint]]];

        msg!("Creating mint acount");

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.collection_mint.to_account_info(),
                    to: ctx.accounts.collection_token_account.to_account_info(),
                    authority: ctx.accounts.collection_mint.to_account_info(),
                },
                signer_seeds,
            ),
            1,
        )?;

        msg!("Creating metadata acount");
        
        create_metadata_accounts_v3(ctx, data, is_mutable, update_authority_is_signer, collection_details)?;

        Ok(())
    }
}
