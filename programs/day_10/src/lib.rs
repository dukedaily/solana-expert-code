use anchor_lang::prelude::*;

declare_id!("C7Qm8zwexesfhk4CSeEKJkasuPsUE1RPQPepsr36wkNS");

#[program]
pub mod day_10 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        let number = get_number();
        msg!("Number: {:?}", number);

        let num = internal_func::internal_func1();
        msg!("Internal num: {:?}", num);

        let msg = some_private_function::private_func();
        msg!("Private function: {:?}", msg);
        Ok(())
    }

    pub mod internal_func {
        pub fn internal_func1() -> u64 {
            return 20;
        }
    }

    pub mod some_private_function {
        use std::string;

        pub(in crate::day_10) fn private_func() -> string::String {
            return "Hello private function!".to_string();
        }
    }
}

// This is a outer module
mod call {
    use crate::day_10;
    pub fn call_internal_func() {
        let num = day_10::internal_func::internal_func1();
        println!("Internal num: {:?}", num);
    }

    pub fn call_private_func() {
        let msg = day_10::some_private_function::private_func();
        println!("Private function: {:?}", msg);
    }
}

pub fn get_number() -> u64 {
    10
}

#[derive(Accounts)]
pub struct Initialize {}
