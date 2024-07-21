use anchor_lang::prelude::*;

declare_id!("6xHvMB26amLT2RuVm4wuX4imZLcnz1LKjea9U3L58pdy");

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
