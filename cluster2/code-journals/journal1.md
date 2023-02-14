# Code journal 1

### prelude
```rust 
use anchor_lang::prelude::*;
```

We can get some of the commonly functionality when using anchor
 * Macros like declare_id!
 * Types like structs Pubkey, Clock ,Account, UncheckedAccount ,
 * Enums like ProgramError

### declare_id!
```rust
declare_id!("ECWPhR3rJbaPfyNFgphnjxSEexbTArc7vxD8fnW6tgKw");
```

macros in rust expand into other code at compile time, 
declaire_id is like entrypoint
declaire_id is where the program id theat will be used for validation should be 
when you init an anchor project there is a default id
you have to build the project and then use anchor keys list to get the key to
use in the declaire_id! macro

### program
```rust
#[program]
```
this attribute is used 


pub mod anchor_program_example {
    use super::*;

    pub fn check_accounts(_ctx: Context<CheckingAccounts>) -> Result<()> {

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CheckingAccounts<'info> {
    payer: Signer<'info>,
    #[account(mut)]
    /// CHECK: This account's data is empty
    account_to_create: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: This account's data is empty
    account_to_change: AccountInfo<'info>,
    system_program: Program<'info, System>,
}