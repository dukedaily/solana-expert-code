use anchor_lang::prelude::*;

declare_id!("36fLt7Dzquf9czaisrnyqMumgJ6vxcD1XWoA7Qcdhpju");

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
