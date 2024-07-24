use anchor_lang::prelude::*;

declare_id!("BDL6WgNCiASzN1YdftWoiLvnVztz5wQ76ktsjb5xZCBj");

#[program]
pub mod day_6 {
    use super::*;

    pub fn age_checker(_ctx: Context<Initialize>, age: u64) -> Result<()> {
        if age >= 18 {
            msg!("You are eligible to vote");
        } else {
            msg!("You are not eligible to vote");
        }

        let result = if age >= 18 {
            "You age is get 18"
        } else {
            "You age is lt 18"
        };
        msg!("{:?}", result);

        return Ok(());
    }
}

#[derive(Accounts)]
pub struct Initialize {}
