use anchor_lang::prelude::*;
use solana_program::log::sol_log_compute_units;

declare_id!("GrtN3e5PKFh84AePChP3RygWue5CDFU5fBXC77YCPEdQ");

#[program]
pub mod day_15 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn more_code(ctx: Context<Initialize>) -> Result<()> {
        let mut v = Vec::new();
        for i in 0..200 {
            v.push(i);
        }
        Ok(())
    }

    pub fn save_units(ctx: Context<Initialize>) -> Result<()> {
        // Measure compute units before any operations
        sol_log_compute_units();

        let mut v1 = Vec::new();
        for i in 0..10 {
            v1.push(i);
        }
        // Measure compute units after creating v1
        sol_log_compute_units();

        let mut v2: Vec<u64> = Vec::new();
        for i in 0..10 {
            v2.push(i);
        }
        // Measure compute units after creating v2
        sol_log_compute_units();

        let mut v3: Vec<i32> = Vec::new();
        for i in 0..10 {
            v3.push(i);
        }
        // Measure compute units after creating v3
        sol_log_compute_units();

        let mut v4: Vec<i64> = Vec::new();
        for i in 0..10 {
            v4.push(i);
        }
        // Measure compute units after creating v3
        sol_log_compute_units();

        let mut v5: Vec<u8> = Vec::new();
        for i in 0..10 {
            v5.push(i);
        }
        // Measure compute units after creating v3
        sol_log_compute_units();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
