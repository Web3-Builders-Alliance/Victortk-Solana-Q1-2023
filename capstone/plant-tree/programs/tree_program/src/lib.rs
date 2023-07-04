use anchor_lang::prelude::*;
use anchor_spl::{
    mint, token::{self,Mint,TokenAccount,Token, },
    associated_token::{AssociatedToken,Create,},   
};
use anchor_lang::solana_program::{pubkey,} ;
use solana_program::{native_token::LAMPORTS_PER_SOL};
use anchor_lang::system_program;
use solana_program::clock::{DEFAULT_TICKS_PER_SLOT,DEFAULT_TICKS_PER_SECOND, SECONDS_PER_DAY} ;
use farm_program::cpi::accounts::{UpdateFarm};
// use tree::cpi::accounts::InitializeFarmer;
// use farmer::program::Farmer as FarmerProgram;
use farm_program::program::FarmProgram;
// use fruit_market::program::FruitMarket as FruitMarketProgram;
// use farmer::{self,Farmer} ;
use farm_program::{self,Farm,LandMeta,CultivarMeta,TreesMeta,Vault} ;
// use fruit_market::{self, FruitMarket, MarketEntry} ;
// mod instructions ;
declare_id!("CUJ8TCeGSKKhqYtZYiBZRghTJvRRRpm9qR2ykX91N1ns");


pub const SLOTS_PER_SECOND: u64=DEFAULT_TICKS_PER_SECOND /DEFAULT_TICKS_PER_SLOT ;

pub const SLOTS_PER_DAY: u64 =SLOTS_PER_SECOND*SECONDS_PER_DAY ;

pub const SLOTS_PER_YEAR: u64 = 5 ;          // SLOTS_PER_DAY*365;

pub const WIDTH_PER_YEAR: u64 = 25000 ; // 25000micrometers 
pub const HEIGHT_PER_YEAR: u64 = 500000;
pub const HEIGHT_PER_SLOT: u64  =  HEIGHT_PER_YEAR / SLOTS_PER_YEAR ;
pub const WIDTH_PER_SLOT: u64  =  WIDTH_PER_YEAR / SLOTS_PER_YEAR ;

pub const ROOT_AREA_GROWTH_RATE:u64  = 1 ;
pub const LEAF_AREA_GROWTH_RATE: u64 = 1 ;

pub const RATE_OF_NUTRIENT_UPTAKE: u64 = 1; 
pub const RATE_OF_WATER_UPTAKE: u64 = 2; 
pub const RATE_OF_FRUIT_INCREMENT: u64 = 1 ;

#[program]
pub mod tree_program {
    use super::*;
    
    pub fn create_tree(ctx: Context<CreateTree> , date: String)-> Result<()> {
        let r_nutrients = &mut *ctx.accounts.required_nutrients;
        r_nutrients.first_check = false; 
    

        let payer = &mut ctx.accounts.payer;
        let trees_meta  = &mut ctx.accounts.trees_meta ;
        trees_meta.tree_count += 1 ;

        let cultivar = &mut ctx.accounts.cultivar ;     

        require!(cultivar.is_initialized, TreeError::CultivarNotInitialized) ;

        cultivar.count = cultivar.count + 1 ;
        // scacity points cultivar.scarcity_points = 
      
       let tree =  &mut ctx.accounts.tree ;
       tree.authority = payer.key();
        tree.cultivar_name =  cultivar.name.clone() ; 
        tree.height = cultivar.init_height;
        tree.girth = cultivar.init_width;
        tree.leaf_area = cultivar.init_leaf_area;
        tree.root_area = cultivar.init_root_area;
        tree.age = 1 ;
        let time =  Clock::get()?.slot ;
        tree.health = 100 ;
        tree.is_alive = true ;
        tree.next_fruit_maturaturation_time =  time + SLOTS_PER_YEAR;
        tree.nft_uri = cultivar.image_uri.clone() ;
        tree.created_date = date ;
        let bump = *ctx.bumps.get("seeds_authority").unwrap() ;
        let p = payer.key();
        let seeds = &[ "seedsauthority".as_bytes(), p.as_ref() ,&[bump]] ;

        let sb = &mut ctx.accounts.seeds_balance; 

        let authority = &mut ctx.accounts.seeds_authority; 

         token::burn(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
               token::Burn{
                   mint: ctx.accounts.fruit_mint.to_account_info().clone(),
                   from: sb.to_account_info().clone() ,
                   authority: authority.to_account_info().clone(), 
                },
                &[&seeds[..]]
            ),
            1
        )?;

    Ok(())  
        
    }    

    pub fn create_cultivar(ctx: Context<CreateCultivar>, cultivar_name:String , height: u64, width: u64, uri: String) -> Result<()>{
      
        let cultivar_meta =  &mut ctx.accounts.cultivar_meta ;
        cultivar_meta.cultivars_count = cultivar_meta.cultivars_count + 1 ;        
        let cultivar = &mut ctx.accounts.cultivar ;
            cultivar.count = 0 ;
            cultivar.name = cultivar_name ;    
            cultivar.init_height= height ;
            cultivar.init_width= width ;
            cultivar.is_initialized = true ;
            cultivar.image_uri = uri ;
      
        let bump = *ctx.bumps.get("fruit_mint_authority").unwrap() ;

        let seeds = &[ "fruitmintauthority".as_bytes(),&[bump]] ;
        let sb = &mut ctx.accounts.seeds_balance;

        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info().clone() ,
                token::MintTo{
                    mint: ctx.accounts.fruit_mint.to_account_info().clone(),
                    to: sb.to_account_info().clone() ,
                    authority: ctx.accounts.fruit_mint_authority.to_account_info().clone(), 
                  } ,
                  &[&seeds[..]]
            ) ,20)
    }
    pub fn create_seed(_ctx: Context<CreateSeed>, name:String) -> Result<()>{
        Ok(())
    } 
    pub fn init_tree_accounts(ctx: Context<InitTreeUpdateAccounts>) -> Result<()>{
        Ok(())
    } 

    pub fn check_and_update(ctx: Context<TreeUpdate>)-> Result<()>{
      
        let r_nutrients = &mut *ctx.accounts.required_nutrients;

        let tree = &mut *ctx.accounts.tree ;


        let current_slot = Clock::get()?.slot;

        if current_slot - tree.planted_time < 11 {
            msg!("Tree too young,");  
            return Ok(())
        };

        if !r_nutrients.first_check{
            msg!("no nutrients");  
            // Err
            return Ok(())
        } 


        if tree.last_consumed_used  {
            msg!("consume nutrients first");  
            // Err
            return err!(TreeError::ConsumeNutrients)
        }

        msg!("After all checks");        

        let tree = &mut *ctx.accounts.tree ;
        // let land_piece = &mut *ctx.accounts.land_piece ;
        
        match tree.update_age() {
            Err(e) =>  return Err(e),
            _ => () 
         }            
    
        tree.grow_expected_fruit_count(r_nutrients.percent_available_potassium,r_nutrients.percent_available_water, r_nutrients.last_check_time);


        tree.grow_leaf_area(r_nutrients.percent_available_nitrogen,r_nutrients.percent_available_water, r_nutrients.last_check_time);


        tree.grow_root_area(r_nutrients.percent_available_phosphorus,r_nutrients.percent_available_water, r_nutrients.last_check_time);

        let bump = *ctx.bumps.get("fruit_mint_authority").unwrap() ;

        let seeds = &[ "fruitmintauthority".as_bytes(),&[bump]] ;
        let fb = &mut ctx.accounts.fruit_balance;

        let is_harvest = match tree.is_harvest_season() { 
            Err(e) => return Err(e),
            Ok(b) => b 
        };  

       
        //here make it a const 1 year and bring const up
        if is_harvest && tree.expected_fruit_count > 1 {
            token::mint_to(
                CpiContext::new_with_signer(
                  ctx.accounts.token_program.to_account_info().clone() ,
                  token::MintTo{
                    mint: ctx.accounts.fruit_mint.to_account_info().clone(),
                    to: fb.to_account_info().clone() ,
                    authority: ctx.accounts.fruit_mint_authority.to_account_info().clone(), 
                  } ,
                  &[&seeds[..]]
                ) ,tree.expected_fruit_count)? ;
         match  tree.set_new_harvest_season() {
            Err(e) =>  return Err(e),
            _ => () 
         }
        } //if true   
              

       match tree.update_size(r_nutrients.percent_available_nitrogen,r_nutrients.percent_available_phosphorus,r_nutrients.percent_available_potassium,r_nutrients.percent_available_water, r_nutrients.last_check_time){
            Err(e) =>  return Err(e),
            _ => () 
         };

        match  tree.set_last_check_time(){
            Err(e) =>  return Err(e),
            _ => () 
        };

        tree.last_consumed_used = true ;
        Ok(())
    }  
    
    pub fn harvest_fruit (ctx: Context<HarvestFruit>, amount: u64  )-> Result<()> {   

       let fruit_vault = &mut ctx.accounts.fruit_vault ;

       let fruit_balance = &mut ctx.accounts.fruit_balance ;

       let tree: &mut Account<Tree> = &mut ctx.accounts.tree ;
       let trees_meta= &mut ctx.accounts.trees_meta.key() ;
       let farmer= &mut ctx.accounts.farmer.key() ;
       let bump  = *ctx.bumps.get("tree").unwrap();

       let seeds=[b"tree",trees_meta.as_ref(),farmer.as_ref(), tree.cultivar_name.as_bytes().as_ref(), tree.created_date.as_bytes().as_ref(),&[bump]] ;
       
		// topEntryFruitBalance,  
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info().clone(), 
                token::Transfer{
                  from: fruit_balance.to_account_info().clone(), 
                  to: fruit_vault.to_account_info().clone(),
                  authority: tree.to_account_info().clone(), 
                },
                &[&seeds[..]]
            ),   
            amount         
        )?;     
      Ok(())
    }
    pub fn water_tree(ctx: Context<TreeUpdate>, amount: u64  )-> Result<()> {   

      let water_mint = &mut ctx.accounts.water_mint ;
        let water_balance = &mut ctx.accounts.water_balance ;
        let nutrient_mint_authority=  &mut ctx.accounts.nutrient_mint_authority ;
        
        let bump = *ctx.bumps.get("nutrient_mint_authority").unwrap() ;

        let seeds = &[ "nutrientmintauthority".as_bytes(), &[bump]] ;
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
               token::MintTo{
                   mint: water_mint.to_account_info().clone(),
                   to: water_balance.to_account_info().clone(),
                   authority: nutrient_mint_authority.to_account_info().clone(), 
                },
                &[&seeds[..]]
            ),
            amount
        )
      
    } 
    pub fn add_nitrogen(ctx: Context<TreeUpdate>, amount: u64  )-> Result<()> {
        let nitrogen_mint = &mut ctx.accounts.nitrogen_mint ;
        let nitrogen_balance = &mut ctx.accounts.nitrogen_balance ;
        let nutrient_mint_authority=  &mut ctx.accounts.nutrient_mint_authority ;
        
        let bump = *ctx.bumps.get("nutrient_mint_authority").unwrap() ;

        let seeds = &[ "nutrientmintauthority".as_bytes(), &[bump]] ;
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
               token::MintTo{
                   mint: nitrogen_mint.to_account_info().clone(),
                   to: nitrogen_balance.to_account_info().clone(),
                   authority: nutrient_mint_authority.to_account_info().clone(), 
                },
                &[&seeds[..]]
            ),
            amount
        )

        // growTree()
    }
    pub fn add_potassium(ctx: Context<TreeUpdate>, amount: u64  )-> Result<()> {
        let potassium_mint = &mut ctx.accounts.potassium_mint ;
        let potassium_balance_account = &mut ctx.accounts.potassium_balance ;
        let nutrient_mint_authority=  &mut ctx.accounts.nutrient_mint_authority ;
        let bump = *ctx.bumps.get("nutrient_mint_authority").unwrap() ;

        let seeds = &[ "nutrientmintauthority".as_bytes(), &[bump]] ;          
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
               token::MintTo{
                   mint: potassium_mint.to_account_info().clone(),
                   to: potassium_balance_account.to_account_info().clone(),
                   authority: nutrient_mint_authority.to_account_info().clone(), 
                },
                &[&seeds[..]]
            ),
            amount
        )
    }
    pub fn add_phosphorus(ctx: Context<TreeUpdate>, amount: u64  )-> Result<()> {
        let phosphorus_mint = &mut ctx.accounts.phosphorus_mint ;
        let phosphorus_balance = &mut ctx.accounts.phosphorus_balance ;
        let nutrient_mint_authority=  &mut ctx.accounts.nutrient_mint_authority ;
        let bump = *ctx.bumps.get("nutrient_mint_authority").unwrap() ;

        let seeds = &[ "nutrientmintauthority".as_bytes(), &[bump]] ;  
        token::mint_to(
            CpiContext::new_with_signer(
               ctx.accounts.token_program.to_account_info(),
               token::MintTo{
                   mint: phosphorus_mint.to_account_info().clone(),
                   to: phosphorus_balance.to_account_info().clone(),
                   authority: nutrient_mint_authority.to_account_info().clone(), 
                },
                &[&seeds[..]]
            ),
            amount
        )
    }
    pub fn consume_nitrogen(ctx: Context<TreeUpdate>, amount: u64 )-> Result<()> {
        let nitrogen_mint = &mut ctx.accounts.nitrogen_mint ;
        let nitrogen_balance = &mut ctx.accounts.nitrogen_balance ;
        let land_piece = &mut ctx.accounts.land_piece.key() ;
        let tree = &mut ctx.accounts.tree.key() ;
        let input_balance = &mut ctx.accounts.input_balance ;      
        let bump = *ctx.bumps.get("input_balance").unwrap() ;
        let seeds = &[ "nutrientbalance".as_bytes(), tree.as_ref(), &[bump]] ;
        token::burn(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Burn{
                   mint: nitrogen_mint.to_account_info().clone(),
                   from: nitrogen_balance.to_account_info().clone(),
                   authority: input_balance.to_account_info().clone(), 
                },
                &[&seeds[..]]
            ),
            amount
        )
    }
    pub fn consume_nutrients(ctx: Context<TreeUpdate>)-> Result<()> {
        
        let phosphorus_mint = &mut ctx.accounts.phosphorus_mint ;
        let phosphorus_balance = &mut ctx.accounts.phosphorus_balance ;
        let land_piece = &mut ctx.accounts.land_piece.key() ;
        let tree = &mut ctx.accounts.tree.key() ;
        let input_balance = &mut ctx.accounts.input_balance ;      
        let bump = *ctx.bumps.get("input_balance").unwrap() ;
        let seeds = &[ "nutrientbalance".as_bytes(), tree.as_ref(), &[bump]] ;
        let potassium_mint = &mut ctx.accounts.potassium_mint ;
        let potassium_balance = &mut ctx.accounts.potassium_balance ;
        let nitrogen_mint = &mut ctx.accounts.nitrogen_mint ;
        let nitrogen_balance = &mut ctx.accounts.nitrogen_balance ;
        let r_nutrients =  &mut *ctx.accounts.required_nutrients ;

        let slot = Clock::get()?.slot ;  

        if slot - r_nutrients.last_check_time > 10 {
           msg!("calculate Required!!");
           return err!(TreeError::CalculateRequired)
        }          

        token::burn(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Burn{
                   mint: nitrogen_mint.to_account_info().clone(),
                   from: nitrogen_balance.to_account_info().clone(),
                   authority: input_balance.to_account_info().clone(), 
                },
                &[&seeds[..]]
            ),
            r_nutrients.nitrogen
        )?;              
       token::burn(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
               token::Burn{
                   mint: potassium_mint.to_account_info().clone(),
                   from: potassium_balance.to_account_info().clone(),
                   authority: input_balance.to_account_info().clone(), 
                },
                &[&seeds[..]]
            ),
             r_nutrients.potassium
        )?;     
              
        token::burn(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
               token::Burn{
                   mint: phosphorus_mint.to_account_info().clone(),
                   from: phosphorus_balance.to_account_info().clone(),
                   authority: input_balance.to_account_info().clone(), 
                },
                &[&seeds[..]]
            ),
          r_nutrients.phosphorus
        )?;
        ctx.accounts.tree.last_consumed_used = false ;
        Ok(())
        
    }
    pub fn consume_phosphorus(ctx: Context<TreeUpdate>, amount: u64 )-> Result<()> {
        let phosphorus_mint = &mut ctx.accounts.phosphorus_mint ;
        let phosphorus_balance = &mut ctx.accounts.phosphorus_balance ;
        let land_piece = &mut ctx.accounts.land_piece.key() ;
        let tree = &mut ctx.accounts.tree.key() ;
        let input_balance = &mut ctx.accounts.input_balance ;      
        let bump = *ctx.bumps.get("input_balance").unwrap() ;
        let seeds = &[ "nutrientbalance".as_bytes(), tree.as_ref(), &[bump]] ;
        token::burn(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
               token::Burn{
                   mint: phosphorus_mint.to_account_info().clone(),
                   from: phosphorus_balance.to_account_info().clone(),
                   authority: input_balance.to_account_info().clone(), 
                },
                &[&seeds[..]]
            ),
            amount
        )
    }
    pub fn consume_potassium(ctx: Context<TreeUpdate>, amount: u64 )-> Result<()> {
        let potassium_mint = &mut ctx.accounts.potassium_mint ;
        let potassium_balance = &mut ctx.accounts.potassium_balance ;
        let land_piece = &mut ctx.accounts.land_piece.key() ;
        let tree = &mut ctx.accounts.tree.key() ;
        let input_balance = &mut ctx.accounts.input_balance ;      
        let bump = *ctx.bumps.get("input_balance").unwrap() ;
        let seeds = &[ "nutrientbalance".as_bytes(),tree.as_ref(), &[bump]] ;
        token::burn(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
               token::Burn{
                   mint: potassium_mint.to_account_info().clone(),
                   from: potassium_balance.to_account_info().clone(),
                   authority: input_balance.to_account_info().clone(), 
                },
                &[&seeds[..]]
            ),
            amount
        )
    }
    pub fn calculate_required( ctx: Context<TreeUpdate>)-> Result<()>{
        let input_balance  = &mut ctx.accounts.input_balance ;    

        let tree = &mut *ctx.accounts.tree ;  

        let r_nutrients = &mut *ctx.accounts.required_nutrients ;
       
        let slot = Clock::get()?.slot ;
        r_nutrients.period = slot - r_nutrients.last_check_time ;
   
        let nitrogen = &ctx.accounts.nitrogen_balance.to_account_info();

        let phosphorus = &ctx.accounts.phosphorus_balance.to_account_info() ;
        let potassium = &ctx.accounts.potassium_balance.to_account_info() ;      
        let water = &ctx.accounts.water_balance.to_account_info() ;        
        let nitrogen_balance = token::accessor::amount(nitrogen)? ;        
        let phosphorus_balance = token::accessor::amount(phosphorus)? ;        
        let potassium_balance = token::accessor::amount(potassium)? ;  

        let water_balance = token::accessor::amount(water)? ;

       r_nutrients.percentage_intake(tree.root_area,  nitrogen_balance.clone() , tree.last_check_time, tree.age,"nitrogen".to_string())? ;
       r_nutrients.percentage_intake(tree.root_area, phosphorus_balance.clone() , tree.last_check_time, tree.age,"phosphorus".to_string())? ;

       r_nutrients.percentage_intake(tree.root_area, potassium_balance.clone() , tree.last_check_time, tree.age,"potassium".to_string())? ;    

       r_nutrients.percentage_intake(tree.root_area, water_balance.clone() , tree.last_check_time, tree.age,"water".to_string())? ;
       r_nutrients.last_check_time = slot ;  
       r_nutrients.first_check = true ;
       
       msg!(
            "What is happening?"
        );
        Ok(())
    }  

    pub fn close_cultivar( ctx: Context<CloseCultivar>) -> Result<()>{
        Ok(())
    }

    pub fn close_tree( ctx: Context<CloseCultivar>) -> Result<()>{
        Ok(())
    }

    pub fn plant (ctx: Context<Plant>, location: [u8;2]) -> Result<()> {            
       if let Ok(()) = ctx.accounts.tree.plant(location) {          
          return Ok(())  
       }
       err!(TreeError::FailedToPlant)     
    }
}


#[derive(Accounts)]
#[instruction(cultivar_name:String)]
pub struct CreateSeed <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(seeds=[b"farm"], bump, seeds::program=farm_program)]
    pub farm: Box<Account<'info,Farm>>,   
    /// CHECK: It is used to derive other accounts which are checked
    pub farmer: UncheckedAccount<'info>, 
    /// CHECK: mint authority for fruit 
    #[account(mut, seeds=[b"fruitmintauthority"], bump,)]
    pub fruit_mint_authority: UncheckedAccount<'info>,

    #[account(seeds=[b"fruitmint",cultivar_name.as_bytes().as_ref()], bump, mint::decimals=9, mint::authority=fruit_mint_authority,)]
    pub fruit_mint: Account<'info, Mint>,

      /// CHECK:authority for seeds balance
    #[account(mut,seeds=[b"seedsauthority", payer.key().as_ref()], bump,)]
    pub seeds_authority: UncheckedAccount<'info>, 

    #[account(init_if_needed, payer=payer ,seeds=[b"seedsbalance", seeds_authority.key().as_ref(),cultivar_name.as_bytes().as_ref()],bump,token::mint=fruit_mint, token::authority=seeds_authority)]
    pub seeds_balance: Box<Account<'info,TokenAccount>>,
    
    pub token_program: Program<'info,Token>,
    pub system_program: Program<'info, System> ,
    // pub tree_program: Program<'info, TreeProgram>,
    pub farm_program: Program<'info, FarmProgram>, 

}
#[derive(Accounts)]
pub struct HarvestFruit <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(seeds=[b"farm"], bump, seeds::program=farm_program)]
    pub farm: Box<Account<'info,Farm>>,    
    /// CHECK: It is used to derive other accounts which are checked
    pub farmer: UncheckedAccount<'info>, 
     /// CHECK: Pda authority 
     #[account(seeds=[b"fruitmintauthority"], bump,)]
    pub fruit_mint_authority: UncheckedAccount<'info>, 
    #[account(mut, seeds=[b"treesmeta",farm.key().as_ref()], bump,seeds::program=farm_program)]
    pub trees_meta: Box<Account<'info, TreesMeta>>,

    #[account(seeds=[b"tree",trees_meta.key().as_ref(),farmer.key().as_ref(),tree.cultivar_name.as_bytes().as_ref(), tree.created_date.as_bytes()], bump,)]
    pub tree: Box<Account<'info, Tree>>,

    #[account(seeds=[b"fruitmint", tree.cultivar_name.as_bytes().as_ref()],bump,mint::decimals=9, mint::authority=fruit_mint_authority)] // different fruits 
    pub fruit_mint: Account<'info, Mint>, 

    #[account(mut,token::mint=fruit_mint, token::authority=farmer)]
    pub fruit_vault: Box<Account<'info,TokenAccount>>,

    #[account(mut, seeds=[b"fruit", tree.key().as_ref()],bump,token::mint=fruit_mint, token::authority=tree)]
    pub fruit_balance: Account<'info,TokenAccount>, 
    
    pub token_program: Program<'info,Token>,
    pub system_program: Program<'info, System> ,
    // pub tree_program: Program<'info, TreeProgram>,
    pub farm_program: Program<'info, FarmProgram>,
}


#[derive(Accounts)]
#[instruction(location: [u8;2])]
pub struct Plant <'info> {
    #[account(seeds=[b"farm"], bump, seeds::program=farm_program)]
    pub farm: Box<Account<'info,Farm>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: It is used to derive other accounts which are checked
    pub farmer: UncheckedAccount<'info>, 
    #[account(mut, seeds=[b"tree",trees_meta.key().as_ref(),farmer.key().as_ref(),tree.cultivar_name.as_bytes().as_ref(), tree.created_date.as_bytes().as_ref()], bump,)]
    pub tree: Box<Account<'info, Tree>>,
    #[account(mut, seeds=[b"treesmeta",farm.key().as_ref()], bump,seeds::program=farm_program)]
    pub trees_meta: Account<'info, TreesMeta>,
    pub farm_program: Program<'info, FarmProgram>,
}

#[derive(Accounts)]
#[instruction(date: String)]
pub struct CreateTree <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds=[b"farm"], bump, seeds::program=farm_program)]
    pub farm: Box<Account<'info,Farm>>,
    /// CHECK: It is used to derive other accounts which are checked
    pub farmer: UncheckedAccount<'info>, 

    #[account(mut, seeds=[b"cultivarmeta",farm.key().as_ref()], bump, seeds::program=farm_program)]
    pub cultivar_meta: Account<'info, CultivarMeta>,

    #[account(mut, seeds=[b"treesmeta",farm.key().as_ref()], bump,seeds::program=farm_program)]
    pub trees_meta: Account<'info, TreesMeta>,

    #[account(mut,seeds=[b"cultivar", cultivar_meta.key().as_ref(), cultivar.name.as_bytes().as_ref()],bump)]
    pub cultivar: Box<Account<'info,Cultivar>>, 
    
    /// CHECK: It is used to derive other accounts which are checked
    #[account(mut,seeds=[b"seedsauthority", payer.key().as_ref()], bump,)]
    pub seeds_authority: UncheckedAccount<'info>, 

    #[account(mut,seeds=[b"seedsbalance", seeds_authority.key().as_ref(),cultivar.name.as_bytes().as_ref()],bump,token::mint=fruit_mint, token::authority=seeds_authority)]
    pub seeds_balance: Box<Account<'info,TokenAccount>>,

    #[account(init,seeds=[b"tree",trees_meta.key().as_ref(),farmer.key().as_ref(),cultivar.name.as_bytes().as_ref(), date.as_bytes().as_ref()], bump, payer=payer, space = 8 + Tree::INIT_SPACE)]
    pub tree: Box<Account<'info, Tree>>,

    /// CHECK: Pda authority 
     #[account(seeds=[b"fruitmintauthority"], bump,)]
    pub fruit_mint_authority: UncheckedAccount<'info>,  

    #[account(mut, seeds=[b"fruitmint", cultivar.name.as_bytes().as_ref()],bump,mint::decimals=9, mint::authority=fruit_mint_authority)] // different fruits 
    pub fruit_mint: Account<'info, Mint>, 

    #[account(init, seeds=[b"fruit", tree.key().as_ref()],bump, payer=payer, token::mint=fruit_mint, token::authority=tree)]
    pub fruit_balance: Box<Account<'info,TokenAccount>>, 

    #[account(init,seeds=[b"nutrientbalance",tree.key().as_ref()],bump, payer=payer, space = 8 + InputBalance::INIT_SPACE)]
    pub input_balance: Box<Account<'info, InputBalance>>,

    #[account(init ,payer=payer, seeds=[b"requirednutrients", tree.key().as_ref()], bump, space= 8 + RequiredNutrients::INIT_SPACE)]
    pub required_nutrients: Box<Account<'info,RequiredNutrients>>,
    
    pub token_program: Program<'info,Token>,
    pub system_program: Program<'info, System> ,
    // pub tree_program: Program<'info, TreeProgram>,
    pub farm_program: Program<'info, FarmProgram>,
    // // pub associated_token_program: Program<'info, AssociatedToken>,
}


#[derive(Accounts)]
#[instruction(date: String)]
pub struct CloseTree <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(seeds=[b"farm"], bump, seeds::program=farm_program)]
    pub farm: Box<Account<'info,Farm>>,
    /// CHECK: It is used to derive other accounts which are checked
    pub farmer: UncheckedAccount<'info>, 
    #[account(mut, seeds=[b"cultivarmeta",farm.key().as_ref()], bump, seeds::program=farm_program)]
    pub cultivar_meta: Account<'info, CultivarMeta>,
    #[account(mut, seeds=[b"treesmeta",farm.key().as_ref()], bump,seeds::program=farm_program)]
    pub trees_meta: Account<'info, TreesMeta>,
    #[account(mut,seeds=[b"cultivar", cultivar_meta.key().as_ref(), cultivar.name.as_bytes().as_ref()],bump)]
    pub cultivar: Box<Account<'info,Cultivar>>,    
    /// CHECK: It is used to derive other accounts which are checked
    #[account(mut,seeds=[b"seedsauthority", payer.key().as_ref()], bump,)]
    pub seeds_authority: UncheckedAccount<'info>, 
    #[account(mut,seeds=[b"seedsbalance", seeds_authority.key().as_ref(),cultivar.name.as_bytes().as_ref()],bump,token::mint=fruit_mint, token::authority=seeds_authority)]
    pub seeds_balance: Box<Account<'info,TokenAccount>>,
    #[account(mut,close=payer ,seeds=[b"tree",trees_meta.key().as_ref(),farmer.key().as_ref(),cultivar.name.as_bytes().as_ref(), date.as_bytes().as_ref()], bump,)]
    pub tree: Box<Account<'info, Tree>>,
    /// CHECK: Pda authority 
     #[account(seeds=[b"fruitmintauthority"], bump,)]
    pub fruit_mint_authority: UncheckedAccount<'info>,  
    #[account(mut, seeds=[b"fruitmint", cultivar.name.as_bytes().as_ref()],bump,mint::decimals=9, mint::authority=fruit_mint_authority)] // different fruits 
    pub fruit_mint: Account<'info, Mint>, 
    #[account(mut,close=payer , seeds=[b"fruit", tree.key().as_ref()],bump,  token::mint=fruit_mint, token::authority=tree)]
    pub fruit_balance: Box<Account<'info,TokenAccount>>, 
    #[account(mut,close=payer ,seeds=[b"nutrientbalance",tree.key().as_ref()],bump,)]
    pub input_balance: Box<Account<'info, InputBalance>>,
    #[account(mut,close=payer, seeds=[b"requirednutrients", tree.key().as_ref()], bump)]
    pub required_nutrients: Box<Account<'info,RequiredNutrients>>, 
    pub farm_program: Program<'info, FarmProgram>,
}

#[derive(Accounts)]
pub struct InitTreeUpdateAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds=[b"farm"], bump, 
    seeds::program=farm_program)]
    pub farm: Box<Account<'info,Farm>>, 

    /// CHECK: famer 
    pub farmer: UncheckedAccount<'info,> ,

    #[account(init_if_needed, payer=payer,seeds=[b"watermint"], bump, mint::decimals=9, mint::authority=nutrient_mint_authority,)]
    pub water_mint: Box<Account<'info, Mint>>,

    #[account(init_if_needed, payer=payer,seeds=[b"nitrogenmint"], bump, mint::decimals=9, mint::authority=nutrient_mint_authority,)]
    pub nitrogen_mint: Box<Account<'info, Mint>> ,

    #[account(init_if_needed, payer=payer,seeds=[b"potassiummint"], bump, mint::decimals=9,
    mint::authority=nutrient_mint_authority, )]
    pub potassium_mint: Box<Account<'info, Mint>> ,

    #[account(init_if_needed, payer=payer,seeds=[b"phosphorusmint"], bump, mint::decimals=9, mint::authority=nutrient_mint_authority,)]
    pub phosphorus_mint: Box<Account<'info, Mint>> ,

    /// CHECK: mint authority pda
    #[account(mut,seeds=[b"nutrientmintauthority"], bump,)]
    pub nutrient_mint_authority: UncheckedAccount<'info,>,

    // #[account(mut,seeds=[b"landmeta", farm.key().as_ref()], bump, seeds::program=farm_program)]
    // pub land_meta: Box<Account<'info,LandMeta>>,

    #[account(seeds=[b"treesmeta",farm.key().as_ref()], bump,seeds::program=farm_program)]
    pub trees_meta: Box<Account<'info, TreesMeta>>,

    #[account(mut,seeds=[b"tree",trees_meta.key().as_ref(),farmer.key().as_ref(), tree.cultivar_name.as_bytes().as_ref(),tree.created_date.as_bytes()], bump, )]
    pub tree: Box<Account<'info, Tree>>,

    #[account(mut, seeds=[b"nutrientbalance",tree.key().as_ref()], bump,)]
    pub input_balance: Box<Account<'info,InputBalance>>,

    #[account(init, payer=payer , seeds=[b"water",input_balance.key().as_ref()], bump, token::mint=water_mint, 
    token::authority=input_balance)]
    pub water_balance: Box<Account<'info,TokenAccount>>,

    #[account(init, payer=payer , seeds=[b"nitrogen",input_balance.key().as_ref()], bump, token::mint=nitrogen_mint, token::authority=input_balance)]
    pub nitrogen_balance: Box<Account<'info,TokenAccount>>,

    #[account(init, payer=payer , seeds=[b"phosphorus",input_balance.key().as_ref()], bump, token::mint=phosphorus_mint, token::authority=input_balance)]
    pub phosphorus_balance: Box<Account<'info,TokenAccount>>,

    #[account(init, payer=payer , seeds=[b"potassium",input_balance.key().as_ref()], bump, token::mint=potassium_mint, token::authority=input_balance)]
    pub potassium_balance: Box<Account<'info,TokenAccount>>,

    pub farm_program: Program<'info,FarmProgram>,
    pub token_program: Program<'info,Token>,
    pub system_program: Program<'info,System> 
} 

#[derive(Accounts)]
pub struct TreeUpdate<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds=[b"farm"], bump, 
    seeds::program=farm_program)]
    pub farm: Box<Account<'info,Farm>>, 

    /// CHECK: famer 
    pub farmer: UncheckedAccount<'info,> ,

    #[account(mut,seeds=[b"watermint"], bump, mint::decimals=9, mint::authority=nutrient_mint_authority,)]
    pub water_mint: Box<Account<'info, Mint>>,

    #[account(mut,seeds=[b"nitrogenmint"], bump, mint::decimals=9, mint::authority=nutrient_mint_authority,)]
    pub nitrogen_mint: Box<Account<'info, Mint>> ,

    #[account(mut,seeds=[b"potassiummint"], bump, mint::decimals=9,
    mint::authority=nutrient_mint_authority, )]
    pub potassium_mint: Box<Account<'info, Mint>> ,

    #[account(mut,seeds=[b"phosphorusmint"], bump, mint::decimals=9, mint::authority=nutrient_mint_authority,)]
    pub phosphorus_mint: Box<Account<'info, Mint>> ,

    /// CHECK: mint authority pda
    #[account(mut,seeds=[b"nutrientmintauthority"], bump,)]
    pub nutrient_mint_authority: UncheckedAccount<'info,>,

    #[account(mut,seeds=[b"landmeta", farm.key().as_ref()], bump, seeds::program=farm_program)]
    pub land_meta: Box<Account<'info,LandMeta>>,

    #[account(seeds=[b"treesmeta",farm.key().as_ref()], bump,seeds::program=farm_program)]
    pub trees_meta: Box<Account<'info, TreesMeta>>,

    #[account(mut,seeds=[b"tree",trees_meta.key().as_ref(),farmer.key().as_ref(), tree.cultivar_name.as_bytes().as_ref(),tree.created_date.as_bytes()], bump, )]
    pub tree: Box<Account<'info, Tree>>,
    
    /// CHECK: farmers' land
    pub land_piece: UncheckedAccount<'info,>,

    #[account(mut, seeds=[b"nutrientbalance",tree.key().as_ref()], bump,)]
    pub input_balance: Box<Account<'info,InputBalance>>,

    #[account(mut, seeds=[b"water",input_balance.key().as_ref()], bump, token::mint=water_mint, 
    token::authority=input_balance)]
    pub water_balance: Box<Account<'info,TokenAccount>>,

    #[account(mut, seeds=[b"nitrogen",input_balance.key().as_ref()], bump, token::mint=nitrogen_mint, token::authority=input_balance)]
    pub nitrogen_balance: Box<Account<'info,TokenAccount>>,

    #[account(mut, seeds=[b"phosphorus",input_balance.key().as_ref()], bump, token::mint=phosphorus_mint, token::authority=input_balance)]
    pub phosphorus_balance: Box<Account<'info,TokenAccount>>,

    #[account(mut,seeds=[b"potassium",input_balance.key().as_ref()], bump, token::mint=potassium_mint, token::authority=input_balance)]
    pub potassium_balance: Box<Account<'info,TokenAccount>>,

    /// CHECK:  pda
    #[account(mut,seeds=[b"fruitmintauthority"], bump,)]
    pub fruit_mint_authority: UncheckedAccount<'info>,

    #[account(mut,seeds=[b"fruitmint",tree.cultivar_name.as_bytes().as_ref()], bump, mint::decimals=9, mint::authority=fruit_mint_authority)]
    pub fruit_mint: Box<Account<'info, Mint>>,

    #[account(mut,seeds=[b"fruit",tree.key().as_ref()], bump, token::mint=fruit_mint, token::authority=tree)]
    pub fruit_balance: Box<Account<'info,TokenAccount>>,

    #[account(mut, seeds=[b"requirednutrients", tree.key().as_ref()], bump,)]
    pub required_nutrients: Box<Account<'info,RequiredNutrients>>,

    #[account(mut,seeds=[b"carbonvault"], 
    bump,seeds::program=farm_program)]
    pub vault: Box<Account<'info,Vault>>,

    pub farm_program: Program<'info,FarmProgram>,
    pub token_program: Program<'info,Token>,
    // pub system_program: Program<'info,System> 
}  

#[derive(Accounts)]
#[instruction(cultivar_name:String)]
pub struct CreateCultivar <'info> {
    #[account(mut)]
    pub payer: Signer<'info>, 

    #[account(seeds=[b"farm"], bump, seeds::program=farm_program)]
    pub farm: Box<Account<'info,Farm>>, 

    /// CHECK: famer 
    pub farmer: UncheckedAccount<'info,> ,

    #[account(mut, seeds=[b"cultivarmeta",
    farm.key().as_ref()], bump,seeds::program=farm_program)]
    pub cultivar_meta: Account<'info, CultivarMeta>,

    #[account(init, payer=payer, seeds=[b"cultivar", cultivar_meta.key().as_ref(), cultivar_name.as_bytes().as_ref()], bump, space = 8 + Cultivar::INIT_SPACE)]
    pub cultivar: Account<'info,Cultivar>,   

    /// CHECK: Pda authority 
     #[account(seeds=[b"fruitmintauthority"], bump,)]
    pub fruit_mint_authority: UncheckedAccount<'info>,

    #[account(init, payer=payer , seeds=[b"fruitmint", 
    cultivar_name.as_bytes().as_ref()],bump, mint::decimals=9, mint::authority=fruit_mint_authority)] // different fruits 
    pub fruit_mint: Account<'info, Mint>, 
   

    /// CHECK: It is used to derive other accounts which are checked
    #[account(mut,seeds=[b"seedsauthority", payer.key().as_ref()], bump,)]
    pub seeds_authority: UncheckedAccount<'info>, 

    #[account(init, payer=payer ,seeds=[b"seedsbalance", seeds_authority.key().as_ref(),cultivar_name.as_bytes().as_ref()],bump,token::mint=fruit_mint, token::authority=seeds_authority)]
    pub seeds_balance: Box<Account<'info,TokenAccount>>,

    pub farm_program: Program<'info,FarmProgram>,
    pub token_program: Program<'info,Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(cultivar_name:String)]
pub struct CloseCultivar <'info> {
    #[account(mut)]
    pub payer: Signer<'info>, 

    #[account(seeds=[b"farm"], bump, seeds::program=farm_program)]
    pub farm: Box<Account<'info,Farm>>, 

    /// CHECK: famer 
    pub farmer: UncheckedAccount<'info,> ,

    #[account(seeds=[b"cultivarmeta",
    farm.key().as_ref()], bump,seeds::program=farm_program)]
    pub cultivar_meta: Account<'info, CultivarMeta>,

    #[account(mut,close=payer, seeds=[b"cultivar", cultivar_meta.key().as_ref(), cultivar_name.as_bytes().as_ref()], bump,)]
    pub cultivar: Account<'info,Cultivar>,   

    /// CHECK: Pda authority 
     #[account(seeds=[b"fruitmintauthority"], bump,)]
    pub fruit_mint_authority: UncheckedAccount<'info>,

    #[account(mut, close=payer, seeds=[b"fruitmint", 
    cultivar_name.as_bytes().as_ref()],bump, mint::decimals=9, mint::authority=fruit_mint_authority)] // different fruits 
    pub fruit_mint: Account<'info, Mint>,
     
    /// CHECK: the authority
    #[account(mut,seeds=[b"seedsauthority", payer.key().as_ref()], bump,)]
    pub seeds_authority: UncheckedAccount<'info>, 

    #[account(mut, close=payer ,seeds=[b"seedsbalance", seeds_authority.key().as_ref(),cultivar_name.as_bytes().as_ref()],bump,token::mint=fruit_mint, token::authority=seeds_authority)]
    pub seeds_balance: Box<Account<'info,TokenAccount>>,
    pub farm_program: Program<'info,FarmProgram>,

    // pub token_program: Program<'info,Token>,
    // pub system_program: Program<'info, System>,
}


#[account]
#[derive(InitSpace)]
pub struct Cultivar {
    pub creator: Pubkey,
    #[max_len(50)]
    pub name: String,
    #[max_len(200)]
    pub image_uri: String,
    pub count:u64,
    pub init_height: u64,
    pub init_width: u64,
    pub init_root_area: u64,
    pub init_leaf_area: u64,
    pub scarcity_points: u64,  
    pub is_initialized: bool , 
}

#[account]
#[derive(Default)]
#[derive(InitSpace)] 
pub struct InputBalance {
}

#[account]
#[derive(InitSpace)]
pub struct Tree {
    pub authority: Pubkey,
    #[max_len(50)]
    pub cultivar_name: String,
    #[max_len(200)]
    pub nft_uri: String,
    pub land_number: u64,
    pub height: u64,
    pub girth: u64,
    pub age: u64 ,
    pub planted_time: u64,
    pub health: u8 ,
    pub last_check_time: u64 ,
    pub last_fruit_update: u64,
    pub next_fruit_maturaturation_time: u64 ,
    pub expected_fruit_count: u64,
    pub is_alive: bool ,
    pub leaf_area: u64 ,
    pub root_area: u64 ,
    pub last_consumed_used: bool,
    pub location: [u8;2],
    #[max_len(32)]
    pub created_date: String ,
    pub is_planted: bool,
}


#[account]
#[derive(InitSpace)]
pub struct RequiredNutrients  {
    pub first_check: bool,
    pub nitrogen: u64 ,
    pub percent_available_nitrogen: u64,
    pub phosphorus: u64,
    pub percent_available_phosphorus: u64,
    pub potassium: u64 ,
    pub percent_available_potassium: u64,
    pub water: u64 ,
    pub percent_available_water: u64,
    pub last_check_time: u64 ,
    pub period: u64
}

impl Tree {

pub fn update_size(&mut self, percent_nitrogen_intake: u64 , percent_phosphorus_intake: u64, percent_potassium_intake: u64, percent_water_intake: u64,recent_check_time: u64 ) -> Result<()>  {
    //calcuate food consumption
    // return food consumption based on tree attributes   

    let period =  recent_check_time - self.last_check_time ;

    let reduction = percent_nitrogen_intake* percent_phosphorus_intake*percent_potassium_intake *percent_water_intake ;

    let height = self.height + (period * HEIGHT_PER_SLOT * reduction) /(100 * 100* 100* 100) ;

    let width =  self.girth + (period * WIDTH_PER_SLOT * reduction) /(100 * 100* 100 * 100) ;

    // should include age 

    let change_height = height - self.height ;
    let change_width = width - self.girth;
    // let leaf_area_growth =  grow_leaves() ;
    if change_height != 0{
        self.height = height ;       
    }

    if change_width !=  0 {
       self.girth = width ;
    }

    Ok(())
}

// pub fn update_girth(&mut self, value: u64){
  
//     self.girth += value;
// }

pub fn update_age(&mut self)-> Result<()> {  

    let slot = Clock::get()?.slot ;   
    let period =  slot - self.planted_time ;
    // let year_time_slots = DEFAULT_TICKS_PER_SECOND / DEFAULT_TICKS_PER_SLOT * SECONDS_PER_DAY * 365 ;
    self.age = period;

    Ok(())
}

pub fn set_last_check_time(&mut self) -> Result<()>{
    self.last_check_time = Clock::get()?.slot ;  //since i am updating using the time from the required_nutrients acount there is no need for this at the moment 
    Ok(())
}

pub fn grow_leaf_area(&mut self, percent_nitrogen_intake: u64, percent_water_intake: u64, recent_check_time: u64){    
 //use some nitrogen 
   let period =  recent_check_time - self.last_check_time ;
   
   let reduction  = ( percent_nitrogen_intake  as f64* percent_water_intake  as f64)/ (100.0 * 100.0) ;

   self.leaf_area += (LEAF_AREA_GROWTH_RATE  as f64 * period  as f64 * reduction) as u64  ;

   msg!("The reduction is now !!!!!! =>>>>>>>>>>>> {:?} ", reduction as f64) ;

   if reduction != 1.0 {
      self.decrease_life(reduction);
   }else {
      self.increase_life() ;
   }
  
}

pub fn grow_root_area(&mut self, percent_phosphorus_intake: u64, percent_water_intake: u64, recent_check_time: u64){ 
    let inc = 25400.0 ;
    let ft = 304800.0 * 1.5 ;

    let root_area = ((self.girth as f64 / inc) * ft) as u64 ;
    let reduction  =( percent_phosphorus_intake as f64 * percent_water_intake  as f64)/ (100.0 * 100.0 );
     msg!("The reduction is =>>>>>>>>>>>> {:?} ", reduction) ;
    self.root_area += (root_area  as f64 * reduction) as u64  ;
      if reduction != 1.0 {
      self.decrease_life(reduction);
   }else {
      self.increase_life() ;
   }
}

pub fn grow_expected_fruit_count(&mut self, percent_potassium_intake: u64, percent_water_intake: u64, recent_check_time: u64) {  
 //use some nitrogen 

  let period =  recent_check_time - self.last_fruit_update ;
  let too_young = SLOTS_PER_YEAR ;

  let young  = SLOTS_PER_YEAR* 10 ;
  let mut age_factor =  0 ; 

  if self.age > too_young && self.age < young {   
     age_factor = 50 ;
  };
  if self.age > young {
     age_factor = 100 ;
  }; 
  let reduction = (percent_potassium_intake as f64 * percent_water_intake as f64  )/ (100.0 * 100.0) ;
  
  let fc: u64 = (RATE_OF_FRUIT_INCREMENT as f64 * period  as f64* reduction  * age_factor as f64 / 100.0).round() as u64 ;
  
   msg!("The reduction is =>>>>>>>>>>>> {:?} ", reduction) ;

   if fc > 0 {
      self.expected_fruit_count += fc ; 
      self.last_fruit_update = recent_check_time ;
   }

   if reduction != 1.0 {
      self.decrease_life(reduction);
   }else {
      self.increase_life() ;
   }
  
  

}

pub fn is_harvest_season(&mut self) -> Result<bool>{    
    match Clock::get() {
        Ok(c) => {
            let s = c.slot ;
            if s>= self.next_fruit_maturaturation_time{
               Ok(true)
            }else{
             Ok(false)
            }
        },
        Err(_e) => {
            err!(TreeError::ClockError)
        }
    }    
}

pub fn set_new_harvest_season(&mut self) -> Result<()> {
     //ticks per second/ ticks per slot * 1 year in seconds should probably get the constants 
    self.expected_fruit_count = 0 ;
    self.next_fruit_maturaturation_time = Clock::get()?.slot + SLOTS_PER_YEAR ;
    Ok(())
}

// fn nutrient_consumption(&mut self){

//     // //Okay Imaginery stuff, 1 token water  1 token phosphorus for the root 
//     // let root_growth = (1,1);
//     // //More imaginary consumption  for leaf we 1
//     // let leaf_growth = (1,1); 

// } 

pub fn plant (&mut self, location : [u8;2] ) -> Result<()> {
    self.location = location ;      
    self.planted_time = Clock::get()?.slot ; 
    self.is_planted = true ;

    Ok(())
}

pub fn increase_life (&mut self, ) {  
    if self.health == 0 {
        self.is_alive = false ;
    } 
    else if self.health < 95 {
       self.health += 5 ;
    }else if self.health > 95 && self.health < 100 {
       self.health = 100 ; 
    }
}

pub fn decrease_life (&mut self,reduction: f64 ){
    self.health = (self.health as f64 * reduction) as u8 ;   
    self.health -= 1 ;
     if self.health == 0 {
        self.is_alive = false; 
    }          
}


}

impl RequiredNutrients {

    pub fn required_uptake (root_area: u64, last_checked_time: u64, age_factor: u64)-> Result<u64> {
        let current_slot = Clock::get()?.slot ;
        msg!("currrent slot:{} ", current_slot );
        let period = current_slot - last_checked_time ;
        msg!("period that has passed since last check:{} ", period) ;
        Ok(RATE_OF_NUTRIENT_UPTAKE * period * age_factor / 100)    // root_area / age    
    }

    pub fn required_water_uptake (root_area: u64, last_checked_time: u64, age_factor: u64)-> Result<u64> {
        let period =  Clock::get()?.slot - last_checked_time ;
        Ok(RATE_OF_WATER_UPTAKE * period * age_factor / 100)  // * root_area / age    
    }

    pub fn percentage_intake(&mut self ,root_area: u64, balance: u64 , last_checked_time: u64, age: u64, nutrient: String)-> Result<()> { 

        let too_young = SLOTS_PER_YEAR ;
        let young  = SLOTS_PER_YEAR* 10 ;

        //Older trees require less nutrients for growth 

        let age_factor = match age {
        x if x < too_young => {
              100
            },
        x  if x < young => {
               70
           },
        _  => {50}
        } ;
  
      match nutrient.as_str() {
            "water" => {                 
                let required = Self::required_water_uptake(root_area,last_checked_time, age_factor)?;
                let (percentage, uptake) = Self::compare(required, balance)? ;
                self.percent_available_water = percentage ;
                self.water = uptake ;
            
             },
            "nitrogen" => {                 
                let required = Self::required_uptake(root_area, last_checked_time, age_factor)? ;
                let (percentage, uptake) = Self::compare(required, balance)? ;
                self.percent_available_nitrogen = percentage ;
                self.nitrogen = uptake ;
            
            } ,
            "potassium" => {                
                 let required = Self::required_uptake(root_area, last_checked_time, age_factor)?;
                 let (percentage, uptake) = Self::compare(required, balance)? ;
                 self.percent_available_potassium = percentage ;
                 self.potassium = uptake ;                        

            } ,
            "phosphorus" => {                 
               let required = Self::required_uptake(root_area, last_checked_time, age_factor)? ;
               let (percentage, uptake) = Self::compare(required, balance)? ;
               self.percent_available_phosphorus = percentage ;
               self.phosphorus = uptake ;             
            } ,
            _ => ()
        };

        Ok(())      

    }

    fn compare (required: u64 , balance: u64) -> Result<(u64,u64)> {
        if required >  balance {
            Ok(((balance * 100/required),balance))                             
        }else {
            Ok((100, required) )
        }
    }

    // pub fn potassium_uptake  (root_area: u64, balance: u64 , last_checked_time: u64, age: u64)-> Result<i64> {
    //     let period =  Clock::get()?.slot - last_checked_time ;

    //     Ok(balance as i64 -  (period as i64* RATE_OF_NUTRIENT_UPTAKE as i64* root_area as i64/ age as i64 ))
       
    // }
    // pub fn phosphorus_uptake  (root_area: u64, balance: u64 , last_checked_time: u64, age: u64)-> Result<i64> {
    //     let period =  Clock::get()?.slot - last_checked_time ;

    //     Ok(balance as i64 -  (period as i64* RATE_OF_NUTRIENT_UPTAKE as i64* root_area as i64/ age as i64 ))
       
    // }
    // pub fn water_uptake  (root_area: u64, balance: u64 , last_checked_time: u64, age: u64)-> Result<i64> {
    //     let period =  Clock::get()?.slot - last_checked_time ;

    //     Ok(balance as i64 -  (period as i64* RATE_OF_WATER_UPTAKE as i64* root_area as i64/ age as i64 ))       
    // }

}

#[error_code]
pub enum TreeError {
    #[msg("The Cultivar account is not initialized!")]
    CultivarNotInitialized,
    ClockError,
    CalculateRequired,
    ConsumeNutrients,
    FailedToPlant,
}


// Calculating critical root radius 
//https://www.acompletetreecare.com/blog/how-to-measure-a-trees-critical-root-zone/
// The general rule of thumb is that for every inch of tree trunk, the radius increases by 1.5 feet. 

//1 inch => 25400 micrometers 

//1 foot => 304800 micromers
