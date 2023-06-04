use anchor_lang::prelude::*;
use anchor_spl::{
    mint, token::{self,Mint,TokenAccount,Token, },
    associated_token::{AssociatedToken,Create,},   
};
use anchor_lang::solana_program::{pubkey} ;

use solana_program::clock::{DEFAULT_TICKS_PER_SLOT,DEFAULT_TICKS_PER_SECOND, SECONDS_PER_DAY} ;

declare_id!("E77dkm2gqBYsPeJzdbvFWDrMP2zvKhnKg4J8uP9UV164");

#[program]
pub mod starter {
    use anchor_lang::system_program;
    use solana_program::native_token::LAMPORTS_PER_SOL;
    use super::*;
    pub fn initialize_farm(ctx: Context<InitializeFarm>) -> Result<()>{
        let land_meta = &mut ctx.accounts.land_meta ;
        land_meta.land_piece_count = 0 ; //to check for initialization        
        let trees_meta = &mut ctx.accounts.trees_meta ;
        trees_meta.tree_count = 0 ; //to check for initialization
        let cultivar_meta =  &mut ctx.accounts.cultivar_meta ;
        cultivar_meta.cultivars_count = 0 ;        
        Ok(())
    }    
    pub fn initialize_farmer(ctx: Context<InitializeFarmer>, user_name: String) -> Result<()> {        
        let payer = &mut ctx.accounts.payer ; 
        let farmer = &mut ctx.accounts.farmer ;
        farmer.name = user_name ;
        farmer.address = payer.key().clone() ;
        farmer.land_count = 0 ;
        farmer.tree_count = 0 ;
        Ok(())      
    }    

}

#[derive(Accounts)]
pub struct InitializeFarmer <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer=payer, seeds=[b"farmer", payer.key().as_ref()], bump, space = 8 + Farmer::INIT_SPACE)]
    pub farmer: Account<'info,Farmer>, 
    // pub token_program: Program<'info,Token>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Farm {
}

#[account]
#[derive(InitSpace)]
pub struct CultivarMeta{
    pub cultivars_count:u64,
}

#[account]
#[derive(InitSpace)]
pub struct LandMeta{
    pub land_piece_count:u64,
}

#[account]
#[derive(InitSpace)]
pub struct TreesMeta{
    pub tree_count:u64,
}

#[account]
#[derive(InitSpace)]
pub struct Cultivar {
    #[max_len(50)]
    pub name: String,
    pub count:u64,
    pub init_height: u64,
    pub init_width: u64,
    pub scarcity_points: u64,  
    pub is_initialized: bool , 
}

#[account]
#[derive(Default)]
#[derive(InitSpace)]
pub struct LandPiece {
    pub owner: Pubkey,
    pub number: u64,
    pub is_planted: bool,
}



#[account]
#[derive(InitSpace)]
pub struct Vault {
    // pub authority: Pubkey 
}

#[account]
#[derive(InitSpace)]
pub struct SeedVault {
    // pub authority: Pubkey 
}

#[account]
#[derive(Default)]
#[derive(InitSpace)]
pub struct Farmer {
    #[max_len(30)]
    pub name: String,
    pub address: Pubkey,
    pub land_count: u64,
    pub tree_count: u64,
}
#[account]
#[derive(Default)]
#[derive(InitSpace)] 
pub struct InputBalance {
}

#[account]
#[derive(InitSpace)]
pub struct Tree {
    #[max_len(50)]
    pub cultivar_name: String,
    pub land_number: u64,
    pub height: u64,
    pub girth: u64,
    pub age: u64 ,
    pub planted_time: u64,
    pub health: u8 ,
    pub last_check_time: u64 ,
    pub next_fruit_maturaturation_time: u64 ,
    pub expected_fruit_count: u64,
    pub is_alive: bool ,
    pub leaf_area: u64 ,
    pub root_area: u64 ,
}


#[account]
#[derive(InitSpace)]
pub struct RequiredNutrients  {
    pub nitrogen: u64 ,
    pub percent_available_nitrogen: u64,
    pub phosphorus: u64,
    pub percent_available_phosphorus: u64,
    pub potassium: u64 ,
    pub percent_available_potassium: u64,
    pub water: u64 ,
    pub percent_available_water: u64,
    pub last_check_time: u64 
}






// #[error_code]
// pub enum TreeError{

// }



