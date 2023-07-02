use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey;
use anchor_lang::system_program;
use anchor_spl::{
    associated_token::{AssociatedToken, Create},
    mint,
    token::{self, Mint, MintTo, Token, TokenAccount},
};
use solana_program::clock::{DEFAULT_TICKS_PER_SECOND, DEFAULT_TICKS_PER_SLOT, SECONDS_PER_DAY};
use solana_program::native_token::LAMPORTS_PER_SOL;

// use farmer::cpi::accounts::UpdateFarmer;
// // use tree::cpi::accounts::InitializeFarmer;
// use farmer::program::Farmer as FarmerProgram;
// use farmer::{self,Farmer} ;

declare_id!("CrYtrU5xK6S98iGQVnyag1XKG9vSYzw2M3Mq4JNHLGSA");

#[program]
pub mod farm_program {
    use super::*;

    pub fn initialize_farm(ctx: Context<InitializeFarm>) -> Result<()> {
        let land_meta = &mut ctx.accounts.land_meta;
        land_meta.land_piece_count = 0; //to check for initialization
        let trees_meta = &mut ctx.accounts.trees_meta;
        trees_meta.tree_count = 0; //to check for initialization
        let cultivar_meta = &mut ctx.accounts.cultivar_meta;
        cultivar_meta.cultivars_count = 0;
        land_meta.x_coord = 0 ;
        land_meta.y_coord = 0 ;
        ctx.accounts.farm.initializer = *ctx.accounts.payer.key ;
        Ok(())
    }

    //   pub fn update_land(ctx: Context<UpdateFarm>) -> Result<()> {
    //         ctx.accounts.land_piece.is_planted = true ;
    //    Ok(())
    //   }
    pub fn update_trees(ctx: Context<UpdateFarm>) -> Result<()> {
        ctx.accounts.trees_meta.tree_count += 1;
        Ok(())
    }
    pub fn update_cultivars(ctx: Context<UpdateFarm>) -> Result<()> {
        ctx.accounts.cultivar_meta.cultivars_count += 1;
        Ok(())
    }

    pub fn close_farm(ctx: Context<CloseFarm>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeFarm<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer=payer, seeds=[b"farm"], bump, space = 8 + Farm::INIT_SPACE)]
    pub farm: Account<'info, Farm>,
    // #[account(init, payer=payer, seeds=[b"fruitmarket"], bump, space =  8 + FruitMarket::INIT_SPACE)]
    // pub fruit_market: Account<'info,FruitMarket>,
    #[account(init, payer=payer, seeds=[b"landmeta", farm.key().as_ref()], bump, space = 8 + LandMeta::INIT_SPACE)]
    pub land_meta: Account<'info, LandMeta>,
    #[account(init, payer=payer,seeds=[b"cultivarmeta",farm.key().as_ref()], bump, space = 8 + CultivarMeta::INIT_SPACE)]
    pub cultivar_meta: Account<'info, CultivarMeta>,
    #[account(init, payer=payer, seeds=[b"treesmeta",farm.key().as_ref()], bump, space = 8 + TreesMeta::INIT_SPACE)]
    pub trees_meta: Account<'info, TreesMeta>,
    #[account(init, payer=payer, seeds=[b"carbonvault"], bump, space = 8 + Vault::INIT_SPACE)]
    pub vault: Account<'info, Vault>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseFarm<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, 
    #[account(mut, close=payer, seeds=[b"farm"], bump,constraint=payer.key== &farm.initializer)]
    pub farm: Account<'info, Farm>,
    // #[account(init, payer=payer, seeds=[b"fruitmarket"], bump, space =  8 + FruitMarket::INIT_SPACE)]
    // pub fruit_market: Account<'info,FruitMarket>,
    #[account(mut, close=payer, seeds=[b"landmeta", farm.key().as_ref()], bump,)]
    pub land_meta: Account<'info, LandMeta>,
    #[account(mut, close=payer,seeds=[b"cultivarmeta",farm.key().as_ref()], bump, )]
    pub cultivar_meta: Account<'info, CultivarMeta>,
    #[account(mut, close=payer, seeds=[b"treesmeta",farm.key().as_ref()], bump, )]
    pub trees_meta: Account<'info, TreesMeta>,
    // #[account(mut, close=payer, seeds=[b"carbonvault"], bump, )]
    // pub vault: Account<'info, Vault>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateFarm<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds=[b"farm"], bump)]
    pub farm: Account<'info, Farm>,
    // #[account(mut, seeds=[b"farmer", payer.key().as_ref(),], bump, seeds::program=farmer_program.key() )]
    // pub farmer: Account<'info,Farmer>,
    #[account(mut,seeds=[b"cultivarmeta",farm.key().as_ref()], bump)]
    pub cultivar_meta: Account<'info, CultivarMeta>,
    #[account(mut,seeds=[b"landmeta", farm.key().as_ref()], bump, )]
    pub land_meta: Account<'info, LandMeta>,
    #[account(mut,seeds=[b"treesmeta",farm.key().as_ref()], bump,)]
    pub trees_meta: Account<'info, TreesMeta>,
    // #[account(mut,seeds=[b"landpiece",land_meta.key().as_ref(),farmer.key().as_ref()], bump,)]
    // pub land_piece: Account<'info, LandPiece>,
    #[account(mut, seeds=[b"carbonvault"], bump,)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Farm {
    pub initializer: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct CultivarMeta {
    pub cultivars_count: u64,
}

#[account]
#[derive(InitSpace)]
pub struct LandMeta {
    pub land_piece_count: u64,
    pub x_coord: u8 ,
    pub y_coord: u8 ,
}

#[account]
#[derive(InitSpace)]
pub struct TreesMeta {
    pub tree_count: u64,
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


impl LandMeta {
    pub fn next_location(&mut self ) -> Result<()>{  
        let x = self.x_coord ;     
        let y = self.y_coord ;   
        if x < 255  {
           self.x_coord += 1 ;
        }
        else if y < 255 {
           self.x_coord = 0 ;
           self.y_coord += 1 ;           
        }else {
           return  err!(FarmError::FarmFullError)             
        }    
        Ok(())         
    }
}

#[error_code]
pub enum FarmError {
    #[msg("The Farm is full")]
    FarmFullError,
}
