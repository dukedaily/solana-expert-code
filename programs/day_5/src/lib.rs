use anchor_lang::prelude::*;

declare_id!("F1FxMXxpFuboFN2wACE8JUaYGEE4TjqHhR8dS9EZ3jQt");

#[program]
pub mod day_5 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
