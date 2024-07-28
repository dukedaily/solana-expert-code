use anchor_lang::prelude::*;

declare_id!("2V4BSWLCWVP5CmrxbcpKG1bqczwgirs43euQtHDJqwDa");

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

pub fn concat(s1: String, s2: String) -> String {
    format!("{}{}", s1, s2)
}

#[program]
pub mod day_7 {
    use super::*;

    pub fn copy_types(ctx: Context<Initialize>) -> Result<()> {
        let a: u32 = 2;
        let b: u32 = 3;
        msg!("{} + {} = {}", a, b, add(a, b));

        let s1 = String::from("hello");
        let s2 = String::from(" world");

        // if s1 and s2 are copied, this could be a huge data transfer
        // if the strings are very long
        msg!("{}{}", s1, s2);
        Ok(())
    }

    pub fn ownership_test(ctx: Context<Initialize>) -> Result<()> {
        let s1 = String::from("hello");
        let res = &s1;

        msg!("s1 under view: {:?}", s1);
        msg!("res view s1: {:?}", res);

        let s2 = String::from("world");
        let res2 = s2;
        msg!("res2 owns s2: {:?}", res2);

        Ok(())
    }

    pub fn borrow_integer(ctx: Context<Initialize>) -> Result<()> {
        let a = 10;
        let b = &a;
        let c = a;

        msg!("a: {:?}", a);
        msg!("b: {:?}", b);
        msg!("c: {:?}", c);
        Ok(())
    }

    pub fn clone_test(ctx: Context<Initialize>) -> Result<()> {
        let mut s1 = String::from("hello");
        let y = &s1;
        // let y = s1.clone();

        s1 = s1 + " world";
        // msg!("y : {:?}", y);
        msg!("s1: {:?}", s1);
        Ok(())
    }

    pub fn generic_type_test(ctx: Context<Initialize>) -> Result<()> {
        let my_values1 = MyValues {
            foo: 42,
            bar: "hello".to_string(),
        };

        let my_values2 = MyValues {
            foo: true,
            bar: [1, 2, 3],
        };

        msg!("my_values1: {:?}", my_values1);
        msg!("my_values2: {:?}", my_values2);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

// derive the debug trait so we can print the struct to the console
#[derive(Debug)]
struct MyValues<T, U> {
    foo: T,
    bar: U,
}

enum MyResultType<T, E> {
    Ok1(T),
    Err1(E),
}
