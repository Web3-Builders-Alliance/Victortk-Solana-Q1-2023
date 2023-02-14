#### Journal 3

//#### anotations inside code

`use anchor_lang::prelude::*; //### here we are including commonly used utils`


`declare_id!("EnjN3cm7xYqYHNUZbQfhJYj5S5RBrSU9tc5aHwQ6LqvT"); `
`//### this tells anchor to use the id here for validation, this is also the entrypoint`


```rust
#[program]//### this tells anchor that the module below is where we have the program logic 

pub mod lever {
    use super::*;    //#### we use the _underscore to tell rust that we wont be using the variable
    pub fn initialize(_ctx: Context<InitializeLever>) -> Result<()> {
        Ok(()) //### all the accounts to initialize are initialize 
               //### in the instruction structs so not much to do here 
    }

    pub fn switch_power(ctx: Context<SetPowerStatus>, name: String) -> Result<()> { 
        //#### the context is where you find the program_id and the accounts 
        //#### required for the instruction
        let power = &mut ctx.accounts.power;//##get mut reference to the power acc
        power.is_on = !power.is_on; //### mutate the value is_on on the 
        //### power account set the field to on if off and off if on 

        //#### the msg! macro is less computationaly expensive that the println! macro
        msg!("{} is pulling the power switch!", &name);

        //### here we just log the status of is_on
        match power.is_on {
            true => msg!("The power is now on."),
            false => msg!("The power is now off!"),
        };

        Ok(())
    }
}
```

`let power = &mut ctx.accounts.power;`
so when we get a mutable reference here it means what ever values we assign 
to the power variable will mutate the original data.


```rust 
#[derive(Accounts)]
pub struct InitializeLever<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub power: Account<'info, PowerStatus>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetPowerStatus<'info> {
    #[account(mut)]
    pub power: Account<'info, PowerStatus>,
}
```



the #[account] is used to add code that is required to serialize and to deserialize the account 

```rust
#[account]
pub struct PowerStatus {//### so PowerStatus is just true or false 
    pub is_on: bool,
}
```