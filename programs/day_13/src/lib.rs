use anchor_lang::prelude::*;

declare_id!("BiWQLQ2PHD34Ynsmn4RcVw3dccmzq3Wv8r8Gds9pGujq");

#[program]
pub mod day_13 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        emit!(MyEvent { value: 41 });
        emit!(MySecondEvent {
            value: 42,
            message: "Hello, Event!".to_string()
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[event]
pub struct MyEvent {
    pub value: u64,
}

#[event]
pub struct MySecondEvent {
    pub value: u64,
    pub message: String,
}
