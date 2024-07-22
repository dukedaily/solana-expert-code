use anchor_lang::prelude::*;

declare_id!("8cK7F1Ku3aFduYUUUE79Ad5WDkQGFM6jC6pUSeiPjzeT");

#[program]
pub mod day_3 {
    use super::*;

    pub fn boaty_mc_boatface(_ctx: Context<ArbitraryStr>) -> Result<()> {
        Ok(())
    }

    pub fn add(_ctx: Context<ArbitraryStr>, a: u64, b: u64) -> Result<()> {
        let sum = a + b;
        msg!("Sum is {}", sum);
        Ok(())
    }

    pub fn sub(_ctx: Context<ArbitraryStr>, a: u64, b: u64) -> Result<()> {
        let difference = a - b;
        msg!("Difference is {}", difference);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ArbitraryStr {}
