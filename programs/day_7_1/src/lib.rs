use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("DjFyT85igx13HFU7uLuqozGaPw6HiQX31PCNLThCCspf");

#[program]
pub mod day_7_1 {
    use super::*;

    pub fn option_test(ctx: Context<Initialize>) -> Result<()> {
        let v = Vec::from([1, 2, 3, 4, 5]);
        assert!(v.iter().max().unwrap() == &5);
        assert!(*v.iter().max().unwrap() == 5);

        Ok(())
    }

    pub fn encode_and_decode(ctx: Context<Initialize>) -> Result<()> {
        let init_person: Person = Person {
            name: "Alice".to_string(),
            age: 30,
        };

        let encoded_data: Vec<u8> = init_person.try_to_vec().unwrap();
        msg!("Encoded data: {:?}", encoded_data);

        let data: Person = decode(ctx, encoded_data)?;
        msg!("Decoded data: {:?}", data);
        Ok(())
    }

    pub fn decode(ctx: Context<Initialize>, encoded_data: Vec<u8>) -> Result<(Person)> {
        let decoded_data: Person = Person::try_from_slice(&encoded_data).unwrap();
        Ok(decoded_data)
        // Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Person {
    name: String,
    age: u32,
}
