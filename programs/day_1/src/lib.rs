use anchor_lang::prelude::*;

declare_id!("G56Ag8rFG6s26TaQpPwwCY9YDv4Mm9PXm3tSXDNP1BZs");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize2(ctx: Context<Initialize>) -> Result<()> {
        // msg!("Greetings from: {:?}", ctx.program_id);
        msg!("Hello, World!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
