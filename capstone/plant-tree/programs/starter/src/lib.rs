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
pub struct Land {
    owner: Pubkey,
    is_planted: bool,
}

#[account]
pub struct Farmer {
    name: String,
    land_areas: Vec<(u64,u64)>,
    trees: Vec<Tree>,
    address: Pubkey
}

#[derive(Accounts)]
pub struct Initialize <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer=payer, seeds=["farmer", payer.key().to_bytes()], bump, space = )]
    pub farmer: Account<'info,Farmer>,
    #[account(mut)]
    pub user: SystemAccount<'info>,
}
