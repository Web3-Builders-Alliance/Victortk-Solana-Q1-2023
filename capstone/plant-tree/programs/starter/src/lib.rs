use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod starter {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        // ctx.accounts.user.
        Ok(())
    }
}


#[derive(Accounts)]
#[instruction(name:String)]
pub struct Initialize <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer=payer, seeds=[b"farmer", payer.key().as_ref()], bump, space = 8 + name.len() + 32)]
    pub farmer: Account<'info,Farmer>,    
    #[account(init, payer=payer, seeds=[b"land", farmer.key().as_ref(),b"1"], bump, space = 8 + name.len() + 32 + 8)]
    pub land: Account<'info,Land>,
     #[account(init, payer=payer, seeds=[b"tree",land.key().as_ref()], bump, space = 8 + name.len() )]
     pub tree: Account<'info, Tree>,

    pub cultivar_meta: Account<'info,CultivarMeta>,
    pub farm_meta: Account<'info,Farm>,
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CultivarMeta {
    
}

#[account]
pub struct Farm {

}

#[account]
pub struct  Vault {

}

#[account]
pub struct Cultivar {
    name: String ,
    init_height: u64,
    init_width: u64,
    scarcity_points: u64,   
}

#[account]
pub struct Tree {
    cultivar: Cultivar,
    height: u64,
    girth: u64,
    age: u64 ,
    planted: u64,
    next_fruit_maturaturation_date: u64 ,
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
    address: Pubkey,
    land_count: u64,
    tree_count: u64,
}