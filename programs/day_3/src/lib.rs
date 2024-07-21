use anchor_lang::prelude::*;

declare_id!("7q2428xGLB9oWr6Z7zPzAjmV249icm9YMiYDdQufAA9e");

#[program]
pub mod day_3 {
    use super::*;

    pub fn boaty_mc_boatface(ctx: Context<ArbitraryStr>) -> Result<()> {
        Ok(())
    }

    pub fn add(ctx: Context<ArbitraryStr>, a: u64, b: u64) -> Result<()> {
        let sum = a + b;
        msg!("Sum is {}", sum);
        Ok(())
    }

    pub fn sub(ctx: Context<ArbitraryStr>, a: u64, b: u64) -> Result<()> {
        let difference = a - b;
        msg!("Difference is {}", difference);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ArbitraryStr {}
