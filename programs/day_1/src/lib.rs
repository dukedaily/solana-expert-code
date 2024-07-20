use anchor_lang::prelude::*;

declare_id!("7yeDb9EZXjcJkWYVCa57Enw3tXy761Z7ZHGp1Sij3KKQ");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize1(ctx: Context<Initialize>) -> Result<()> {
        // msg!("Greetings from: {:?}", ctx.program_id);
        msg!("Hello, World!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
