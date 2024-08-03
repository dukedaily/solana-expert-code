use anchor_lang::prelude::*;

declare_id!("BnV6jJjxmxsfjrzDic4dNuz1a9Gyos18adaMgDt4a7ya");
const MAX_POINTS1: u32 = 100_000;
const MAX_POINTS2: u32 = 200_000;

static mut x: u32 = 100;
// let x=100;

#[program]
pub mod day_6 {
    use std::collections::HashMap;

    use super::*;

    pub fn age_checker(_ctx: Context<Initialize>, age: u64) -> Result<()> {
        // 1. standard way
        if age >= 18 {
            msg!("You are eligible to vote");
        } else {
            msg!("You are not eligible to vote");
        }

        // 2. special way
        let result = if age >= 18 {
            "You age is >= 18"
        } else {
            "You age is < 18"
        };
        msg!("{:?}", result);

        // 3. using match
        match age {
            1 => {
                msg!("You are 1 year old");
            }
            2 | 3 => {
                msg!("You are 2 or 3 years old");
            }
            4..=17 => {
                msg!("You are between 4 and 17(inclusive) years old");
            }
            _ => {
                msg!("You are 18 years old or older");
            }
        }
        Ok(())
    }

    pub fn for_loops(_ctx: Context<Initialize>) -> Result<()> {
        // 1. standard way
        for i in 0..5 {
            msg!("Standard for loop: {}", i);
        }

        // 2. customize step
        for i in (0..5).step_by(2) {
            msg!("Customize step: {}", i);
        }
        Ok(())
    }

    pub fn fixed_array(_ctx: Context<Initialize>) -> Result<()> {
        let arr_x: [u64; 5] = [1, 2, 3, 4, 5];
        let a1 = arr_x[0];
        let a2: u64 = arr_x[1];

        // compile error, cos bydefault array is immutable
        // a1_x[1]= 10；

        let mut arr_y = [1, 2, 3, 4, 5];
        // it works, cos we make it mutable during declaration
        arr_y[1] = 10;

        for i in 0..arr_y.len() {
            msg!(": {}", arr_y[i]);
        }
        Ok(())
    }

    pub fn dynamic_array(_ctx: Context<Initialize>) -> Result<()> {
        let mut d_arr: Vec<u32> = Vec::new();
        d_arr.push(1);
        d_arr.push(2);
        d_arr.push(3);

        let first = d_arr[0];
        d_arr[1] = 10;
        msg!("d_arr[1]: {}", d_arr[1]);
        Ok(())
    }

    pub fn mapping_test(_ctx: Context<Initialize>) -> Result<()> {
        // let mut map: HashMap<String, String> = HashMap::new();
        let mut map1 = HashMap::new();
        // map.insert("name", "John");
        // map.insert("age", "25");
        map1.insert("name", "John");
        map1.insert("age", "25");

        let name = map1.get("name");
        let age = map1.get("age");
        msg!("Name: {:?}", name);
        msg!("Age: {:?}", age);

        // let mut map2: HashMap<String, u32> = HashMap::new();
        let mut map2 = HashMap::new();
        map2.insert("age", 25);
        let age = map2.get("age");
        let age_index = map2["age"];
        msg!("Age: {:?}, {}", age, age_index);
        Ok(())
    }

    pub fn struct_test(_ctx: Context<Initialize>) -> Result<()> {
        struct Person {
            name: String,
            age: u32,
        }

        let p1 = Person {
            // name: "John", // cannot compile, need to use to_string()
            name: "John".to_string(),
            age: 25,
        };

        msg!("Name: {}, Age: {}", p1.name, p1.age);
        Ok(())
    }

    const MAX_POINTS3: u32 = 300_000;

    pub fn constant_test(_ctx: Context<Initialize>) -> Result<()> {
        const MAX_POINTS4: u32 = 400_000;
        msg!("MAX_POINTS1: {} ", MAX_POINTS1);
        msg!("MAX_POINTS2: {} ", MAX_POINTS2);
        msg!("MAX_POINTS3: {} ", MAX_POINTS3);
        msg!("MAX_POINTS4: {} ", MAX_POINTS4);

        const MAX_POINTS2: u32 = 500_000;
        msg!("Local MAX_POINTS2: {} ", MAX_POINTS2);
        Ok(())
    }

    pub fn usize_test(_ctx: Context<Initialize>) -> Result<()> {
        // The pointer-sized unsigned integer type.
        // The size of this primitive is how many bytes it takes to reference any location in memory.
        // For example, on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.
        let mut a: usize = 100;
        let b: u32 = 200;
        let d: u128 = 400;
        msg!("size of a: {}", std::mem::size_of_val(&a));
        msg!("size of b: {}", std::mem::size_of_val(&b));

        a = d as usize;
        msg!("new size of a: {}", std::mem::size_of_val(&a));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
