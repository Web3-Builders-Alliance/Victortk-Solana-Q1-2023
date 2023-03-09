use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod starter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {        
        let payer = &mut ctx.accounts.payer ;
        let cultivar = &mut ctx.accounts.tree.cultivar.name ;
        let land = &mut ctx.accounts.land ;
        land.owner = payer.key() ;
        land.is_planted = true ;
        Ok(())
    }
}


#[derive(Accounts)]
#[instruction(name:String)]
pub struct Initialize <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer=payer, seeds=[b"farmer", payer.key().as_ref()], bump, space = 8 +Farmer::INIT_SPACE , constraint = farmer.name.len() < 30)]
    pub farmer: Account<'info,Farmer>,    
    #[account(init, payer=payer, seeds=[b"land", farmer.key().as_ref(),b"1"], bump, space = 8 + name.len() + 32 + 8)]
    pub land: Account<'info,Land>,
     #[account(init, payer=payer, seeds=[b"tree",land.key().as_ref()], bump, space = 8 + name.len() )]
    pub tree: Account<'info, Tree>,
    pub cultivar_meta: Account<'info,CultivarMeta>,
    pub farm_meta: Account<'info,Farm>,
    #[account(init_if_needed, payer=payer, seeds=[b"carbonsolvault"], bump, space = 8 + Vault::INIT_SPACE)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct CultivarMeta{
    name: String,
    pub count:u64,
}


#[account]
#[derive(InitSpace)]
pub struct Farm {
    pub tree_count: u64,
    pub cultivar_count: u64 ,
    pub cultivar_names: CultivarNames, 
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, )]
#[derive(InitSpace)]
pub enum CultivarNames{
    Butternut = 1,
    Mango,
    Guava,
    Eucalyptus 
}

#[account]
#[derive(InitSpace)]
pub struct Vault {
 pub authority: Pubkey 
}

#[account]
#[derive(InitSpace)]
pub struct Cultivar {
    name: String ,
    init_height: u64,
    init_width: u64,
    scarcity_points: u64,   
}

#[account]
#[derive(InitSpace)]
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
#[derive(InitSpace)]
pub struct Land {
    owner: Pubkey,
    is_planted: bool,
}

#[account]
#[derive(Default)]
#[derive(InitSpace)]
pub struct Farmer {
    name: String,
    address: Pubkey,
    land_count: u64,
    tree_count: u64,
}