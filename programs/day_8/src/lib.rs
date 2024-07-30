use anchor_lang::prelude::*;

declare_id!("DnAyn23aQJC3LL74G1FHwksnHCoFyefNNRchHjWgHc79");

#[program]
pub mod day_8 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("hello");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
