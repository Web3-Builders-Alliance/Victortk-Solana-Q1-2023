use anchor_lang::prelude::*;
use anchor_spl::{
    mint, token::{self,Mint,TokenAccount,Token, MintTo},
    associated_token::{AssociatedToken,Create,},   
};
use anchor_lang::solana_program::{pubkey,} ;
use solana_program::{native_token::LAMPORTS_PER_SOL};
use anchor_lang::system_program;
use solana_program::clock::{DEFAULT_TICKS_PER_SLOT,DEFAULT_TICKS_PER_SECOND, SECONDS_PER_DAY} ;

// use farmer::cpi::accounts::UpdateFarmer;
// // use tree::cpi::accounts::InitializeFarmer;
// use farmer::program::Farmer as FarmerProgram;
// use farmer::{self,Farmer} ;

declare_id!("6DDP3hohHprxPNUWVtwpK89QAzcB27Fk4NSCgcq368P6");

#[program]
pub mod farm {
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

 
//   pub fn update_land(ctx: Context<UpdateFarm>) -> Result<()> {
//         ctx.accounts.land_piece.is_planted = true ;
//    Ok(())
//   }
  pub fn update_trees(ctx: Context<UpdateFarm>) -> Result<()> {
    ctx.accounts.trees_meta.tree_count += 1 ;
    Ok(())
  }
  pub fn update_cultivars(ctx: Context<UpdateFarm>) -> Result<()> {
    ctx.accounts.cultivar_meta.cultivars_count += 1 ;
    Ok(())
  }  
    
}


#[derive(Accounts)]
pub struct InitializeFarm <'info> {
     #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer=payer, seeds=[b"farm"], bump, space = 8 + Farm::INIT_SPACE)]
    pub farm: Account<'info,Farm>,
    // #[account(init, payer=payer, seeds=[b"fruitmarket"], bump, space =  8 + FruitMarket::INIT_SPACE)]
    // pub fruit_market: Account<'info,FruitMarket>,
    #[account(init, payer=payer, seeds=[b"landmeta", farm.key().as_ref()], bump, space = 8 + LandMeta::INIT_SPACE)]
    pub land_meta: Account<'info,LandMeta>,
    #[account(init, payer=payer,seeds=[b"cultivarmeta",farm.key().as_ref()], bump, space = 8 + CultivarMeta::INIT_SPACE)]
    pub cultivar_meta: Account<'info, CultivarMeta>,
    #[account(init, payer=payer, seeds=[b"treesmeta",farm.key().as_ref()], bump, space = 8 + TreesMeta::INIT_SPACE)]
    pub trees_meta: Account<'info, TreesMeta>,
    #[account(init, payer=payer, seeds=[b"carbonvault"], bump, space = 8 + Vault::INIT_SPACE)]
    pub vault: Account<'info,Vault>,
    pub token_program: Program<'info,Token>,
    pub system_program: Program<'info,System>,
}


#[derive(Accounts)]
pub struct UpdateFarm <'info> { 
    #[account(mut)]
    pub payer: Signer<'info>,      
    #[account(mut, seeds=[b"farm"], bump)]
    pub farm: Account<'info,Farm>, 
    // #[account(mut, seeds=[b"farmer", payer.key().as_ref(),], bump, seeds::program=farmer_program.key() )]
    // pub farmer: Account<'info,Farmer>,
    #[account(mut,seeds=[b"cultivarmeta",farm.key().as_ref()], bump)]
    pub cultivar_meta: Account<'info, CultivarMeta>, 
    #[account(mut,seeds=[b"landmeta", farm.key().as_ref()], bump, )]
    pub land_meta: Account<'info,LandMeta>,
    #[account(mut,seeds=[b"treesmeta",farm.key().as_ref()], bump,)]
    pub trees_meta: Account<'info, TreesMeta>,
    // #[account(mut,seeds=[b"landpiece",land_meta.key().as_ref(),farmer.key().as_ref()], bump,)]
    // pub land_piece: Account<'info, LandPiece>,
    #[account(mut, seeds=[b"carbonvault"], bump,)]
    pub vault: Account<'info, Vault>,
 
    pub system_program: Program<'info, System>
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
pub struct Vault {
    // pub authority: Pubkey 
}

// #[account]
// #[derive(InitSpace)]
// pub struct SeedVault {
//     // pub authority: Pubkey 
// }




