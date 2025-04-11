#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;
use anchor_spl::metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3};
use anchor_spl::token_interface::{mint_to, MintTo};

pub mod instructions;
pub use instructions::*;

declare_id!("4VNuGptng1b6vwb8fAMsy5Y5Kg7rKsaZT6bTXL9rrK8U");

#[constant]
pub const NAME: &str = "Token lottery Ticket #";
#[constant]
pub const SYMBOL: &str = "TLT";
#[constant]
pub const URI:&str="https://media.istockphoto.com/id/1500283713/vector/cinema-ticket-on-white-background-movie-ticket-on-white-background.jpg?s=2048x2048&w=is&k=20&c=FVKTESRh2yCgIhKNOX1p3WliXW6jf9iGcErciapIIJw=";

#[program]
pub mod tokenlottery {

    use anchor_spl::metadata::{
        create_master_edition_v3,
        mpl_token_metadata::types::{CollectionDetails, Creator, DataV2},
        sign_metadata, CreateMasterEditionV3, SignMetadata,
    };

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
                &signer_seeds,
            ),
            1,
        )?;

        msg!("Creating metadata acount");

        create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: ctx.accounts.metadata.to_account_info(),
                    mint: ctx.accounts.collection_mint.to_account_info(),
                    mint_authority: ctx.accounts.collection_mint.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    update_authority: ctx.accounts.collection_mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                &signer_seeds,
            ),
            DataV2 {
                name: NAME.to_string(),
                symbol: SYMBOL.to_string(),
                uri: URI.to_string(),
                seller_fee_basis_points: 0,
                creators: Some(vec![Creator {
                    address: ctx.accounts.collection_mint.key(),
                    verified: false,
                    share: 100,
                }]),
                collection: None,
                uses: None,
            },
            true,
            true,
            Some(CollectionDetails::V1 { size: 0 }),
        )?;

        msg!("Creating master edition");

        create_master_edition_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMasterEditionV3 {
                    payer: ctx.accounts.payer.to_account_info(),
                    edition: ctx.accounts.master_edition.to_account_info(),
                    mint: ctx.accounts.collection_mint.to_account_info(),
                    mint_authority: ctx.accounts.collection_mint.to_account_info(),
                    update_authority: ctx.accounts.collection_mint.to_account_info(),
                    metadata: ctx.accounts.metadata.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                &signer_seeds,
            ),
            Some(0),
        )?;

        msg!("Verifying  collection");

        sign_metadata(CpiContext::new_with_signer(
            ctx.accounts.collection_mint.to_account_info(),
            SignMetadata{
                creator:ctx.accounts.collection_mint.to_account_info(),
                metadata:ctx.accounts.metadata.to_account_info(),
            },
            &signer_seeds,
        ))?;
        
        Ok(())
    }
}
