use anchor_lang::prelude::*;

declare_id!("BHmuwneQbAui2zgvvWXPaefcuHJrDnSdw9pS1WeUJsdw");

#[program]
pub mod day_2 {
    use super::*;

    pub fn initialize2(ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        // msg!("Greetings from: {:?}", ctx.program_id);
        msg!("message: {:?}", message);
        msg!("You sent {} and {}", a, b);
        Ok(())
    }

    pub fn array(ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
    }

    pub fn overflow_unsafe(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        // test1: overflow silently
        let x_unsafe: u64 = a - b;

        msg!("x_unsafe: {}", x_unsafe);
        Ok(())
    }

    pub fn overflow_safe(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let x_safe: u64 = a.checked_sub(b).unwrap();
        msg!("x_safe: {}", x_safe);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
