use anchor_lang::prelude::*;

declare_id!("6roYSTL7e16ErEnQ9TDBYqVkkW9YCWDs3547ETibyANY");

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
