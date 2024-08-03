use anchor_lang::prelude::*;

declare_id!("2SCX7gj5ByjWvin16BNNDjZxEcsikX15DWMMccc7EVXD");

#[program]
pub mod day_11 {
    use super::*;
    use chrono::*;

    pub fn get_timestamp(ctx: Context<Initialize>) -> Result<()> {
        let clock: Clock = Clock::get()?;
        msg!("Current Unix Timestamp: {}", clock.unix_timestamp);
        Ok(())
    }

    pub fn get_day_of_week(ctx: Context<Initialize>) -> Result<()> {
        let clock: Clock = Clock::get()?;
        let time_stamp = clock.unix_timestamp;

        // let date_time = chrono::NaiveDateTime::from_timestamp(time_stamp, 0);
        let date_time = DateTime::from_timestamp(time_stamp, 0);
        msg!(
            "Day of the week: {:?}",
            date_time.expect("REASON").weekday()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
