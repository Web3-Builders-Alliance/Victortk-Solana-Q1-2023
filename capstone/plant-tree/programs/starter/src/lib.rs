use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod starter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // ctx.accounts.user.
        Ok(())
    }
}


#[account]
pub struct Tree {
    name: String,
    location: Land 
}

#[account]
#[derive(Default)]
pub struct Land {
    owner: Pubkey,
    is_planted: bool,
}

#[account]
#[derive(Default)]
pub struct Farmer {
    name: String,
    address: Pubkey
}

#[derive(Accounts)]
#[instruction(name:String)]
pub struct Initialize <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer=payer, seeds=[b"farmer", payer.key().as_ref()], bump, space = 8 + name.len() + 32)]
    pub farmer: Account<'info,Farmer>,
    pub system_program: Program<'info, System>,
}
