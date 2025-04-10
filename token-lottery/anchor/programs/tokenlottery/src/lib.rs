#![allow(clippy::result_large_err)]
pub mod instructions;

use anchor_lang::prelude::*;
declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod tokenlottery {
    use crate::instructions::Instructions::InitializeConfig;

    use super::*;
    pub fn intialize_config(ctx:Context<InitializeConfig>)->Result<()>{
      Ok(())
    }

}
