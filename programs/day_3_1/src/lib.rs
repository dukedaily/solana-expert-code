use anchor_lang::prelude::*;

declare_id!("49zJroisjGYVCBnnwRav9bCGScxLqhpw4dvPGhYbfWkU");

#[program]
pub mod day_3_1 {
    use super::*;

    pub fn non_empty_account_example(_ctx: Context<ArbitraryStr>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ArbitraryStr<'info> {
    signer: Signer<'info>,
    another_signer: Signer<'info>,
}