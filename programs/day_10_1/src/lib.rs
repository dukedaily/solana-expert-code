use anchor_lang::prelude::*;

declare_id!("Hya7FfvnDRhEB1cRNZAtfeu5TVjxCJR1BEQK81btLUCs");
pub mod calculate;

#[program]
pub mod day_10_1 {
    use super::*;

    pub fn myAdd(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let res = calculate::add(a, b);
        msg!("Result: {:?}", res); // a: 10, b: 20, res: 30
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
