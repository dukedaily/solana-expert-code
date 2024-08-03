use anchor_lang::prelude::*;

declare_id!("9qfovSZQtZ3VkgCTiDDWuMCpkPyxt3HAcHh19MiX6pGU");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize1(_ctx: Context<Initialize>) -> Result<()> {
        // msg!("Greetings from: {:?}", _ctx.program_id);
        msg!("Hello, World!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
