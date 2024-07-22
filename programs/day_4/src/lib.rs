use anchor_lang::prelude::*;

declare_id!("FtRZjdbYvZVW59CVSe94F6XiaahpCuEuByJqw4bH9Ray");

#[program]
pub mod day_4 {
    use super::*;

    // this function will return an error, but the tx won't revert, differ from solidity!
    pub fn limit_range(_ctx: Context<LimitRange>, a: u64) -> Result<()> {
        if a < 10 {
            return err!(MyError::AisTooSmall);
        }
        if a > 100 {
            return err!(MyError::AisTooBig);
        }
        msg!("Result = {}", a);
        Ok(())
    }

    // the require! macro is a shortcut for the above function, won't revert the tx neither
    pub fn limit_range_require(_ctx_then: Context<LimitRange>, a: u64) -> Result<()> {
        require!(a >= 10, MyError::AisTooSmall);
        require!(a <= 100, MyError::AisTooBig);
        msg!("Result = {}", a);
        Ok(())
    }

    // won't revert and the msg! macro will not print, cos it return err!() instead of Ok(())
    pub fn funcError(_ctx: Context<LimitRange>) -> Result<()> {
        msg!("Will this print when return err!() ?");
        return err!(MyError::AlwaysErrors);
        // output: this msg will not print
    }

    // will print the msg! macro, cos it return Ok(())
    pub fn funcOK(_ctx: Context<LimitRange>) -> Result<()> {
        msg!("Will this print when return OK() ?");
        // return err!(MyError::AlwaysErrors);
        return Ok(());

        // output: this msg will print
    }
}

#[derive(Accounts)]
pub struct LimitRange {}

#[error_code]
pub enum MyError {
    #[msg("a is too small")]
    AisTooSmall,
    #[msg("a is too big")]
    AisTooBig,
    #[msg("Always errors")]
    AlwaysErrors,
}
