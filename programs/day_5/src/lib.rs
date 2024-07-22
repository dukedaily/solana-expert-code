use anchor_lang::prelude::*;

declare_id!("HNdiq7jWsmQ33GPmRd95NR3Momr8pcUVx3GB7MuJMhU");

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
