use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("5cesgJ7ENbdnpqPQdnNci2dHAVYxKyP2rxSp37SxesWF");

#[program]
pub mod day_18 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    // new code
    pub fn set(ctx: Context<Set>, x: u64) -> Result<()> {
        ctx.accounts.my_storage_set.x = x;
        Ok(())
    }
}

// new code
#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut, seeds=[], bump)]
    pub my_storage_set: Account<'info, MyStorage>,
}

#[account]
pub struct MyStorage {
    x: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer=signer, space=size_of::<MyStorage>()+8,seeds=[], bump)]
    pub my_storage_account: Account<'info, MyStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
