use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod starter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, name: String, tree_cultivar: Cultivar ) -> Result<()> {        
        let payer = &mut ctx.accounts.payer ; 
        let farmer = &mut ctx.accounts.farmer ;
        farmer.name = name ;
        farmer.address = payer.key().clone() ;
        farmer.land_count = 1 ;
        farmer.tree_count = 1 ;

        // let cultivar = &mut ctx.accounts.tree.cultivar.name ;
        
        let tree =  &mut ctx.accounts.tree ;
        tree.cultivar = tree_cultivar.clone() ;
        tree.height = tree_cultivar.init_height;
        tree.girth = tree_cultivar.init_width;
        tree.age = 0 ;
        let time =  Clock::get()?.slot ;
        tree.planted = time ;
        //ticks per second/ ticks per slot * 1 year in seconds should probably get the constants 
        tree.next_fruit_maturaturation_date =  time + 5 * 30 * 60 * 24 * 365 ; 

        let cultivar_meta =  &mut ctx.accounts.cultivar_meta ;
        // name: String,
        // pub count:u64,

        let farm_meta =  &mut ctx.accounts.farm_meta ;
        //  pub tree_count: u64,
        //  pub cultivar_count: u64 ,
        //  pub cultivar_names: TreeName, 


        let vault =  &mut ctx.accounts.vault ;
        //transfer sol to vault

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
    #[max_len(30)]
    name: String,
    pub count:u64,
}


#[account]
#[derive(InitSpace)]
pub struct Farm {
    pub tree_count: u64,
    pub cultivar_count: u64 ,
    pub cultivar_names: TreeName, 
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, )]
#[derive(InitSpace)]
pub enum TreeName{
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
    name: TreeName ,
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
    #[max_len(30)]
    name: String,
    address: Pubkey,
    land_count: u64,
    tree_count: u64,
}