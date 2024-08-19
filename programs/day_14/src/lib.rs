use anchor_lang::prelude::*;

declare_id!("559zmVS2NrnjZGpu3ALRHp2M8WRaZMGZ2rNx4STSASM6");

// NOTE: Replace with you own address(Public key)
const OWNER: &str = "HjU6xSZme7ER6Qhk841nczwXijBZ9e1GWLqdPxW6gS9w";

#[program]
pub mod day_14 {
    use super::*;

    pub fn single_signer(ctx: Context<Initialize>) -> Result<()> {
        let signer1: &mut Signer = &mut ctx.accounts.signer1;
        // Your logic here

        msg!("Signer1: {:?}", *signer1.key);
        Ok(())
    }

    pub fn multiple_signer2(ctx: Context<Initialize>) -> Result<()> {
        let signer1: &mut Signer = &mut ctx.accounts.signer1;
        let signer2: &Signer = &ctx.accounts.signer2;
        // Your logic here

        msg!("Signer1: {:?}", *signer1.key);
        msg!("Signer2: {:?}", *signer2.key);
        Ok(())
    }

    #[access_control(check(&ctx))]
    pub fn restrict_to_owner(ctx: Context<OnlyOwner>) -> Result<()> {
        // Your logic here
        msg!("Hello, i'm the true king, arise!");
        Ok(())
    }
}

fn check(ctx: &Context<OnlyOwner>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.signer_account.key(),
        OWNER.parse::<Pubkey>().unwrap(),
        OnlyOwnerError::NotOwner
    );
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer1: Signer<'info>,
    pub signer2: Signer<'info>,
}

#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    signer_account: Signer<'info>,
}

#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call this function")]
    NotOwner,
}
