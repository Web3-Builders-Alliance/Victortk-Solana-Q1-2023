
```rust
use anchor_lang::prelude::*;
use lever::cpi::accounts::SetPowerStatus; //### here we get the Instruction 
//##struct so that we can use it to create the accounts required for the CPIContext
use lever::program::Lever; 
use lever::{self, PowerStatus};
```

declare_id!("ABoYG2GWbzLgnnGhK2pUGNupzKoYe7UGk2idrAXbstAS");


```rust
#[program]
mod hand {
    use super::*;

    //### all this account is doing is to call the lever program via CPIx
    //### with the account it got and the name of the person pulling the lever
    pub fn pull_lever(ctx: Context<PullLever>, name: String) -> anchor_lang::Result<()> {
        
        // Hitting the switch_power method on the lever program
        //### switch power processing function takes in two parameters 
        //### it takes in the a Context and name string
        //### to create the Context we use CPIContext::new method 
        lever::cpi::switch_power(
            CpiContext::new(
                //###The context is made up of the program id and accounts required by the processor
                
                ctx.accounts.lever_program.to_account_info(), 

                // Using the accounts context struct from the lever program
                //
                let cpi_accounts = SetPowerStatus {
                    power: ctx.accounts.power.to_account_info(),
                };
            ), 
            name //###name is required by the lever, it is printed out to the 
            //#### loggs 
        )
    }
}
```


```rust 
#[derive(Accounts)]
pub struct PullLever<'info> {
    #[account(mut)]
    pub power: Account<'info, PowerStatus>, //## this account is passed to the lever program by CPI
    pub lever_program: Program<'info, Lever>, //### here we get the program to call via cpi
}

```

`pub power: Account<'info, PowerStatus>,`
the PowerStatus account is imported from the lever Program 
anchor uses so that it knows how to deserialize the account. 
