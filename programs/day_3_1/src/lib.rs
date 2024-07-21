use anchor_lang::prelude::*;

declare_id!("Ab8YxzsdF4WQnbzxEReNF7qa7P7Jz8zBnXjbiqRaHg4x");

#[program]
pub mod day_3_1 {
    use super::*;

    pub fn non_empty_account_example(ctx: Context<ArbitraryStr>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ArbitraryStr<'info> {
    signer: Signer<'info>,
    another_signer: Signer<'info>,
}
