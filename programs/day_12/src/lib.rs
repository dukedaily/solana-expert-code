use anchor_lang::prelude::*;

declare_id!("8FErGg2jmveugDusoUAwtNw6HWzCHkBtmjc555tcHCcF");

#[program]
pub mod day_12 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let epoch = EpochSchedule::get()?;
        msg!("Epoch: {:?}", epoch);
        Ok(())
    }

    pub fn rent_test(ctx: Context<Initialize>) -> Result<()> {
        let rent = Rent::get()?;
        msg!("Rent: {:?}", rent);
        Ok(())
    }

    pub fn get_stake_history(ctx: Context<Initialize>) -> Result<()> {
        let arr = [ctx.accounts.stake_history.clone()];
        let acc_iter = &mut arr.iter();
        let sh_sysvar_info = next_account_info(acc_iter)?;
        msg!("Stake History Sysvar: {:?}", sh_sysvar_info);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK:
    pub stake_history: AccountInfo<'info>,
}
