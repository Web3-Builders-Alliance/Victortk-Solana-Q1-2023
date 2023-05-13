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
    pub fn buy_land(ctx: Context<BuyLand>) -> Result<()> {
        let vault =  &mut ctx.accounts.vault ;
        let payer = &mut ctx.accounts.payer ; 
        let farmer = &mut ctx.accounts.farmer ;
        let land_meta = &mut ctx.accounts.land_meta ;
        let land_piece = &mut ctx.accounts.land_piece ;
        land_meta.land_piece_count += 1 ; //to check for initialization
        farmer.land_count += 1 ;
        land_piece.owner = farmer.key() ;
        land_piece.number = land_meta.land_piece_count ;
        //transfer sol to vault
        let lamports = LAMPORTS_PER_SOL/4 ;
        system_program::transfer(
            CpiContext::new(
               ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: payer.to_account_info(),
                    to: vault.to_account_info(),
            },  
        ), lamports)
    }
    pub fn plant_tree(ctx: Context<PlantTree>, cultivar_name:String)-> Result<()> {
        let farmer = &mut  ctx.accounts.farmer ;
        farmer.tree_count += 1 ;
        let trees_meta  = &mut ctx.accounts.trees_meta ;
        trees_meta.tree_count += 1 ;

        let cultivar = &mut ctx.accounts.cultivar ;
        // assert!(cultivar.is_initialized) ;

        cultivar.count = cultivar.count + 1 ;
        //scacity points cultivar.scarcity_points = 

        let land_piece = &mut ctx.accounts.land_piece ;     
        land_piece.is_planted = true ;   

        let tree =  &mut ctx.accounts.tree ;
        tree.cultivar_name =  cultivar.name.clone() ;
        tree.land_number = land_piece.number ;
        tree.cultivar_name = cultivar_name ;
        tree.height = cultivar.init_height;
        tree.girth = cultivar.init_width;
        tree.age = 1 ;
        let time =  Clock::get()?.slot ;
        tree.planted_time = time ;
        tree.last_check_time = time ;
        tree.health = 100 ;
        tree.is_alive = true ;
        //ticks per second/ ticks per slot * 1 year in seconds should probably get the constants  + 5 * 30 * 60 * 24 * 365 
        tree.next_fruit_maturaturation_time =  time + 1;        
        Ok(())
    }
   
    pub fn create_cultivar(ctx: Context<CreateCultivar>, name:    String , height: u64, width: u64) -> Result<()>{
        let cultivar_meta =  &mut ctx.accounts.cultivar_meta ;
        cultivar_meta.cultivars_count = cultivar_meta.cultivars_count + 1 ;

        let market = &mut ctx.accounts.fruit_market ;
        let market_entry  = &mut ctx.accounts.market_entry ;

        //   maker: Pubkey ,
        //   count: u64 ,
        //   next_maker: Option<Pubkey>,

        market_entry.maker = *ctx.program_id ;
        market_entry.count = 0 ;
        market_entry.next_maker = Some(*ctx.program_id) ;
        
        market.bottom_maker =Some(*ctx.program_id) ;
        market.top_maker = Some(*ctx.program_id);  
        market.total_fruit_balance = 0 ;      
        
        let cultivar = &mut ctx.accounts.cultivar ;
            cultivar.count = 0 ;
            cultivar.name = name ;    
            cultivar.init_height= height ;
            cultivar.init_width= width ;
            cultivar.is_initialized = true ;
        Ok(())
    }
    pub fn check_and_update(ctx: Context<TreeUpdate>)-> Result<()>{
        msg!("In=================================================>In");
        let r_nutrients = &mut *ctx.accounts.required_nutrients;

        let current_slot = Clock::get()?.slot;

        if current_slot < 11 {
            return Ok(())
        };

        if r_nutrients.last_check_time < current_slot -10 {
            // Err
            return Ok(())
        }         

        let nutrient_authority =  &mut ctx.accounts.nutrient_mint_authority ;
        let n_mint = &mut *ctx.accounts.nitrogen_mint;
        let p_mint = &mut *ctx.accounts.phosphorus_mint;
        let k_mint = &mut *ctx.accounts.potassium_mint;
        let w_mint = &mut *ctx.accounts.water_mint;
      
        let input_balance  = &mut ctx.accounts.input_balance ;

        let tree = &mut *ctx.accounts.tree ;
        let land_piece = &mut *ctx.accounts.land_piece ;
        
        let nitrogen = &ctx.accounts.nitrogen_balance.to_account_info();
        
        let phosphorus = &ctx.accounts.phosphorus_balance.to_account_info() ;
        let potassium = &ctx.accounts.potassium_balance.to_account_info() ;
      
        let water = &ctx.accounts.water_balance.to_account_info() ;

        
        let nitrogen_balance = token::accessor::amount(nitrogen)? ;
        
        let phosphorus_balance = token::accessor::amount(phosphorus)? ;
        
        let potassium_balance = token::accessor::amount(potassium)? ;

        let water_balance = token::accessor::amount(water)? ;
        let bump = *ctx.bumps.get("nutrient_mint_authority").unwrap() ;

        let seeds = &[ "nutrientmintauthority".as_bytes(), &[bump]] ;

        let tree_key = tree.key() ;
        let land_piece_key = land_piece.key() ;
        // // seeds=[b"nutrientbalance",land_piece.key().as_ref(),tree.key().as_ref()], bump,      
       
        msg!("r_nutrients.percent_available_potassium: {} , r_nutrients.percent_available_water: {} ,r_nutrients.last_check_time: {}", r_nutrients.percent_available_potassium,r_nutrients.percent_available_water, r_nutrients.last_check_time) ;

        tree.grow_expected_fruit_count(r_nutrients.percent_available_potassium,r_nutrients.percent_available_water, r_nutrients.last_check_time);

        tree.grow_leaf_area(r_nutrients.percent_available_nitrogen,r_nutrients.percent_available_water, r_nutrients.last_check_time);
        tree.grow_root_area(r_nutrients.percent_available_phosphorus,r_nutrients.percent_available_water, r_nutrients.last_check_time);

        let bump = *ctx.bumps.get("fruit_mint_authority").unwrap() ;

        let seeds = &[ "fruitmintauthority".as_bytes(),&[bump]] ;
        let fb = &mut ctx.accounts.fruit_balance;
        if tree.is_harvest_season()? { 
         
            token::mint_to(
                CpiContext::new_with_signer(
                  ctx.accounts.token_program.to_account_info().clone() ,
                  token::MintTo{
                    mint: ctx.accounts.fruit_mint.to_account_info().clone(),
                    to: fb.to_account_info().clone() ,
                    authority: ctx.accounts.fruit_mint_authority.to_account_info().clone(), 
                  } ,
                  &[&seeds[..]]
                ) ,1000000000)? ;
        //  tree.expected_fruit_count
        tree.set_new_harvest_season()?;
        } //if true   
        fb.reload()?;

        msg!("The new fb account is  {:?} ,", fb) ;    
        msg!("The new fb balance is  {} ,", fb.amount) ; 
        

        tree.update_age()? ;
        tree.update_size(r_nutrients.percent_available_nitrogen,r_nutrients.percent_available_phosphorus,r_nutrients.percent_available_potassium,r_nutrients.percent_available_water, r_nutrients.last_check_time)? ;
        tree.set_last_check_time()? ; 
        Ok(())
    }

    pub fn water_tree(ctx: Context<TreeUpdate>, amount: u64  )-> Result<()> {     
        let water_mint = &mut ctx.accounts.water_mint ;
        let water_balance_account = &mut ctx.accounts.water_balance ;
        let nutrient_mint_authority=  &mut ctx.accounts.nutrient_mint_authority ; 

        let bump = *ctx.bumps.get("nutrient_mint_authority").unwrap() ;

        let seeds = &[ "nutrientmintauthority".as_bytes(), &[bump]] ;
      
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
               token::MintTo{
                   mint: water_mint.to_account_info().clone(),
                   to: water_balance_account.to_account_info().clone(),
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
        let seeds = &[ "nutrientbalance".as_bytes(),land_piece.as_ref(), tree.as_ref(), &[bump]] ;
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
    pub fn consume_phosphorus(ctx: Context<TreeUpdate>, amount: u64 )-> Result<()> {
        let phosphorus_mint = &mut ctx.accounts.phosphorus_mint ;
        let phosphorus_balance = &mut ctx.accounts.phosphorus_balance ;
        let land_piece = &mut ctx.accounts.land_piece.key() ;
        let tree = &mut ctx.accounts.tree.key() ;
        let input_balance = &mut ctx.accounts.input_balance ;      
        let bump = *ctx.bumps.get("input_balance").unwrap() ;
        let seeds = &[ "nutrientbalance".as_bytes(),land_piece.as_ref(), tree.as_ref(), &[bump]] ;
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
        let seeds = &[ "nutrientbalance".as_bytes(),land_piece.as_ref(), tree.as_ref(), &[bump]] ;


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

        let nitrogen = &ctx.accounts.nitrogen_balance.to_account_info();
        let phosphorus = &ctx.accounts.phosphorus_balance.to_account_info() ;
        let potassium = &ctx.accounts.potassium_balance.to_account_info() ;      
        let water = &ctx.accounts.water_balance.to_account_info() ;        
        let nitrogen_balance = token::accessor::amount(nitrogen)? ;        
        let phosphorus_balance = token::accessor::amount(phosphorus)? ;        
        let potassium_balance = token::accessor::amount(potassium)? ;        
        let water_balance = token::accessor::amount(water)? ;

        let (n_percent, r_n) = input_balance.percentage_intake(tree.root_area,  nitrogen_balance.clone() , tree.last_check_time, tree.age,false)? ;
        r_nutrients.nitrogen = r_n ;
        r_nutrients.percent_available_nitrogen = n_percent ;

        let (p_percent, r_p) =input_balance.percentage_intake(tree.root_area, phosphorus_balance.clone() , tree.last_check_time, tree.age,false)? ;
        r_nutrients.phosphorus = r_p ;
        r_nutrients.percent_available_phosphorus = p_percent ;

        let (k_percent, r_k) =input_balance.percentage_intake(tree.root_area, potassium_balance.clone() , tree.last_check_time, tree.age,false)? ;
        r_nutrients.potassium = r_k ;
         r_nutrients.percent_available_potassium = k_percent ;

        let (w_percent, r_w) =input_balance.percentage_intake(tree.root_area, water_balance.clone() , tree.last_check_time, tree.age,true)? ;

        r_nutrients.water = r_w ; 
        r_nutrients.percent_available_water = w_percent ;
        r_nutrients.last_check_time = Clock::get()?.slot ;  
        Ok(())
    }  

    pub fn list_fruits(ctx:Context<ListFruit>, amount: u64) ->Result<()>{
        // transfer tokens quantity  from tree to the market token account 
      
        // fruit_mint_authority
        let fruit_mint_authority = &mut ctx.accounts.fruit_mint_authority ;
        // fruit_mint
         let fruit_mint = &mut ctx.accounts.fruit_mint ;
        // fruit_vault
         let entry_fruit_balance = &mut ctx.accounts.entry_fruit_balance ;
        // fruit_balance
         let fruit_balance = &mut *ctx.accounts.fruit_balance ;
         let farmer = &mut ctx.accounts.farmer ;
         let payer = &mut ctx.accounts.payer.key() ;     

         
         //increment the market volume info 
         let market = &mut *ctx.accounts.fruit_market ;

         msg!("market b4 was  :{:?} ", market.top_maker);
         let market_entry  = &mut *ctx.accounts.market_entry ;

         let current_top_market_entry  = &mut **ctx.accounts.current_top_market_entry ;
         
        //   maker: Pubkey ,
        //   count: u64 ,
        //   next_maker: Option<Pubkey>,
        msg!("the maker is : ======>> {}", payer ) ;

        market_entry.maker = *payer ;
        market_entry.count = amount ;

        market_entry.next_maker = Some(current_top_market_entry.maker) ;       
        
        market.top_maker = Some(market_entry.maker);  
        market.total_fruit_balance += market_entry.count ;

        let bump = *ctx.bumps.get("farmer").unwrap() ;
        let seeds = &["farmer".as_bytes(), payer.as_ref(), &[bump]] ;
       
        let k =  pubkey::Pubkey::find_program_address(&["farmer".as_bytes(), payer.as_ref()], ctx.program_id);


        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info().clone(), 
                token::Transfer{
                  from: fruit_balance.to_account_info().clone(), 
                  to: entry_fruit_balance.to_account_info().clone(),
                  authority: farmer.to_account_info().clone(), 
                },
                &[&seeds[..]]
            ),   
            amount         
        )?;


        // market.reload()?; 

        msg!("market is now :{:?} ", market.top_maker);

        Ok(())
    }

    pub fn buy_fruits(ctx:Context<BuyFruit>, cultivar_name:String) ->Result<()>{
        // // seedVault,
        // let seed_vault: &mut Box<Account<SeedVault>> = &mut ctx.accounts.seed_vault ;

		// seedsBalance,
        let seeds_balance = &mut ctx.accounts.seeds_balance ;
		// fruit_market,
        let fruit_market = &mut ctx.accounts.fruit_market ;
		// current_top_market_entry,
        let current_top_market_entry = &mut ctx.accounts.current_top_market_entry ;
         
        let current_top_market_entry = &mut ctx.accounts.current_top_market_entry ;

        let market = &mut ctx.accounts.fruit_market ;

        let top_entry_fruit_balance  = &mut ctx.accounts.top_entry_fruit_balance ;

        let fruit_mint_authority  = &mut ctx.accounts.fruit_mint_authority ;

        //   maker: Pubkey ,
        //   count: u64 ,
        //   next_maker: Option<Pubkey>,

        let bump  = *ctx.bumps.get("fruit_mint_authority").unwrap();
        let seeds = &["fruitmintauthority".as_bytes(), &[bump]] ;
		// topEntryFruitBalance,  
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info().clone(), 
                token::Transfer{
                  from: top_entry_fruit_balance.to_account_info().clone(), 
                  to: seeds_balance.to_account_info().clone(),
                  authority: fruit_mint_authority.to_account_info().clone(), 
                },
                &[&seeds[..]]
            ),   
            1         
        )?;        
        Ok(())
    }

}


#[derive(Accounts)]
#[instruction(cultivar_name:String)]
pub struct BuyFruit<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds=[b"farmer", payer.key().as_ref()], bump, )]
    pub farmer: Account<'info,Farmer>, 

    #[account(seeds=[b"farm"], bump)]
    pub farm: Box<Account<'info,Farm>>,

    #[account(mut,seeds=[b"cultivarmeta",
    farm.key().as_ref()], bump,)]
    pub cultivar_meta: Box<Account<'info, CultivarMeta>>,

    #[account(mut,seeds=[b"cultivar", cultivar_meta.key().as_ref(), cultivar_name.as_bytes().as_ref()], bump)]
    pub cultivar: Box<Account<'info,Cultivar>>,   
    
    /// CHECK: fruit mint authourity pda 
    #[account(mut, seeds=[b"fruitmintauthority"], bump,)]
    pub fruit_mint_authority: UncheckedAccount<'info>,

    #[account(seeds=[b"fruitmint",cultivar_name.as_bytes().as_ref()], bump, mint::decimals=9, mint::authority=fruit_mint_authority)]
    pub fruit_mint: Account<'info, Mint>,

    // #[account(init_if_needed, payer=payer , seeds=[b"seedvault",farmer.key().as_ref()], bump,space=8 + SeedVault::INIT_SPACE )] //token::authority=program. does that happen by default 
    // pub seed_vault: Box<Account<'info,SeedVault>>, 

    #[account(init_if_needed, payer=payer ,seeds=[b"seedsbalance",farmer.key().as_ref()], bump, token::mint=fruit_mint, token::authority=farmer)]
    pub seeds_balance: Box<Account<'info,TokenAccount>>,
    
    #[account(mut,seeds=[b"fruitmarket", cultivar_name.as_bytes().as_ref()], bump)]
    pub fruit_market: Account<'info,FruitMarket>,

    #[account(mut,seeds=[b"marketentry", fruit_market.key().as_ref(),fruit_market.top_maker.unwrap().as_ref()], bump)]
    pub current_top_market_entry: Account<'info,MarketEntry>,

    #[account(mut,seeds=[b"fruit",current_top_market_entry.key().as_ref()], bump, token::mint=fruit_mint,)]
    pub top_entry_fruit_balance: Account<'info,TokenAccount>,

    pub system_program: Program<'info,System>,
    pub token_program: Program<'info,Token>,
}


#[derive(Accounts)]
pub struct ListFruit<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,    
    #[account(seeds=[b"farm"], bump)]
    pub farm: Box<Account<'info,Farm>>,
    #[account(mut, seeds=[b"farmer", payer.key().as_ref()], bump, )]
    pub farmer: Account<'info,Farmer>,
    // #[account(seeds=[b"fruitvaultauthority"], bump)]
    // /// CHECK: this is  a PDA authrity acc
    // pub fruit_vault_authority: UncheckedAccount<'info>, 
    #[account( seeds=[b"treesmeta",farm.key().as_ref()], bump, )]
    pub trees_meta: Box<Account<'info, TreesMeta>>,
    #[account( seeds=[b"tree",trees_meta.key().as_ref(),farmer.key().as_ref()], bump, )]
    pub tree: Box<Account<'info, Tree>>,
    /// CHECK: fruit mint authourity pda 
    #[account(seeds=[b"fruitmintauthority"], bump,)]
    pub fruit_mint_authority: UncheckedAccount<'info>,
    #[account(seeds=[b"fruitmint", 
    tree.cultivar_name.as_bytes()],bump, mint::decimals=9, mint::authority=fruit_mint_authority)] // different fruits 
    pub fruit_mint: Box<Account<'info, Mint>>,
  
    #[account(mut, seeds=[b"fruit",tree.key().as_ref()], bump, token::mint=fruit_mint, token::authority=farmer)]
    pub fruit_balance: Box<Account<'info,TokenAccount>>,

    #[account(mut, seeds=[b"fruitmarket", tree.cultivar_name.as_bytes()], bump)]
    pub fruit_market: Account<'info,FruitMarket>,
    #[account(init_if_needed, payer=payer, seeds=[b"marketentry", fruit_market.key().as_ref(),payer.key().as_ref()],space =  8 + MarketEntry::INIT_SPACE,bump)]
    pub market_entry: Account<'info,MarketEntry>,
    
    #[account(init_if_needed, payer=payer,seeds=[b"marketentry", fruit_market.key().as_ref(),fruit_market.top_maker.unwrap().as_ref()], bump, space =  8 + MarketEntry::INIT_SPACE)]
    pub current_top_market_entry: Box<Account<'info,MarketEntry>>,

    #[account(init, payer=payer, seeds=[b"fruit",market_entry.key().as_ref()], bump, token::mint=fruit_mint, token::authority=fruit_mint_authority)]
    pub entry_fruit_balance: Account<'info,TokenAccount>, //entry you are making
    
    //   #[account(init_if_needed, payer= payer , seeds=[b"fruit",tree.cultivar_name.as_bytes()], bump,token::mint=fruit_mint,token::authority=fruit_vault_authority )] //token::authority=program. does that happen by default cultivar_name.as_bytes().as_ref()
    // pub fruit_vault: Box<Account<'info,TokenAccount>>,    
    
    
    pub system_program: Program<'info,System>,
    pub token_program: Program<'info,Token>,
}

#[account]
#[derive(Default)]
#[derive(InitSpace)]
pub struct MarketEntry {
  maker: Pubkey ,
  count: u64 ,
  next_maker: Option<Pubkey>,
}

#[account]
#[derive(Default)]
#[derive(InitSpace)]
pub struct FruitMarket {
   top_maker: Option<Pubkey> ,
   bottom_maker: Option<Pubkey> ,
   total_fruit_balance: u64 ,
   price: u64 ,
}

#[derive(Accounts)]
pub struct TreeUpdate<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(seeds=[b"farm"], bump)]
    pub farm: Box<Account<'info,Farm>>,  
    #[account(seeds=[b"farmer", payer.key().as_ref()], bump, )]
    pub farmer: Box<Account<'info,Farmer>>,
    #[account(mut,seeds=[b"watermint"], bump, mint::decimals=9, mint::authority=nutrient_mint_authority)]
    pub water_mint: Box<Account<'info, Mint>> ,
    #[account(mut,seeds=[b"nitrogenmint"], bump, mint::decimals=9, mint::authority=nutrient_mint_authority)]
    pub nitrogen_mint: Box<Account<'info, Mint>> ,
    #[account(mut,seeds=[b"potassiummint"], bump, mint::decimals=9,
    mint::authority=nutrient_mint_authority)]
    pub potassium_mint: Box<Account<'info, Mint>> ,
    #[account(mut,seeds=[b"phosphorusmint"], bump, mint::decimals=9, mint::authority=nutrient_mint_authority)]
    pub phosphorus_mint: Box<Account<'info, Mint>> ,
    /// CHECK: mint authority pda
    #[account(seeds=[b"nutrientmintauthority"], bump,)]
    pub nutrient_mint_authority: UncheckedAccount<'info,>,
    #[account(seeds=[b"landmeta", farm.key().as_ref()], bump)]
    pub land_meta: Box<Account<'info,LandMeta>>,
    #[account(seeds=[b"treesmeta",farm.key().as_ref()], bump, )]
    pub trees_meta: Box<Account<'info, TreesMeta>>,
    #[account(mut,seeds=[b"tree",trees_meta.key().as_ref(),farmer.key().as_ref()], bump, )]
    pub tree: Box<Account<'info, Tree>>,

    #[account(seeds=[b"landpiece",land_meta.key().as_ref(),farmer.key().as_ref()], bump,)]
    pub land_piece:Box<Account<'info, LandPiece>>,

    #[account(mut, seeds=[b"nutrientbalance",land_piece.key().as_ref(),tree.key().as_ref()], bump,)]
    pub input_balance: Box<Account<'info,InputBalance>>,

    #[account(init_if_needed, payer=payer , seeds=[b"water",input_balance.key().as_ref()], bump, token::mint=water_mint, token::authority=input_balance)]
    pub water_balance: Box<Account<'info,TokenAccount>>,

    #[account(init_if_needed, payer=payer , seeds=[b"nitrogen",input_balance.key().as_ref()], bump, token::mint=nitrogen_mint, token::authority=input_balance)]
    pub nitrogen_balance: Box<Account<'info,TokenAccount>>,

    #[account(init_if_needed, payer=payer , seeds=[b"phosphorus",input_balance.key().as_ref()], bump, token::mint=phosphorus_mint, token::authority=input_balance)]
    pub phosphorus_balance: Box<Account<'info,TokenAccount>>,

    #[account(init_if_needed, payer=payer , seeds=[b"potassium",input_balance.key().as_ref()], bump, token::mint=potassium_mint, token::authority=input_balance)]
    pub potassium_balance: Box<Account<'info,TokenAccount>>,

    /// CHECK:  pda
    #[account( seeds=[b"fruitmintauthority"], bump,)]
    pub fruit_mint_authority: UncheckedAccount<'info>,

    #[account(mut,seeds=[b"fruitmint",tree.cultivar_name.as_ref()], bump, mint::decimals=9, mint::authority=fruit_mint_authority)]
    pub fruit_mint: Box<Account<'info, Mint>> ,

    #[account(mut,seeds=[b"fruit",tree.key().as_ref()], bump, token::mint=fruit_mint, token::authority=farmer)]
    pub fruit_balance: Box<Account<'info,TokenAccount>>,
    #[account(mut, seeds=[b"requirednutrients", tree.key().as_ref()], bump,)]
    pub required_nutrients: Box<Account<'info,RequiredNutrients>>,

    pub token_program: Program<'info,Token>,
    pub system_program: Program<'info,System> 
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

    #[account(init, payer=payer, seeds=[b"watermint"], bump, mint::decimals=9, mint::authority=nutrient_mint_authority)]
    pub water_mint: Account<'info, Mint> ,
    #[account(seeds=[b"nutrientmintauthority"], bump)]
    /// CHECK: pda signer 
    pub nutrient_mint_authority: UncheckedAccount<'info,>,
    #[account(init, payer=payer, seeds=[b"nitrogenmint"], bump, mint::decimals=9, mint::authority=nutrient_mint_authority)]
    pub nitrogen_mint: Box<Account<'info, Mint>> ,

    #[account(init, payer=payer,seeds=[b"potassiummint"], bump, mint::decimals=9,
    mint::authority=nutrient_mint_authority)]
    pub potassium_mint: Box<Account<'info,Mint>> ,

    #[account(init, payer=payer,seeds=[b"phosphorusmint"], bump, mint::decimals=9, mint::authority=nutrient_mint_authority)]
    pub phosphorus_mint: Box<Account<'info,Mint>> ,

    #[account(init, payer=payer, seeds=[b"carbonvault"], bump, space = 8 + Vault::INIT_SPACE)]
    pub vault: Account<'info,Vault>,

    pub token_program: Program<'info,Token>,

    pub system_program: Program<'info,System>,
}    


#[derive(Accounts)]
#[instruction(cultivar_name:String)]
pub struct CreateCultivar <'info> {
    #[account(mut)]
    pub payer: Signer<'info>, 

    #[account(seeds=[b"farm"], bump)]
    pub farm: Account<'info,Farm>,

    #[account(seeds=[b"cultivarmeta",
    farm.key().as_ref()], bump,)]
    pub cultivar_meta: Account<'info, CultivarMeta>,

    #[account(init, payer=payer, seeds=[b"cultivar", cultivar_meta.key().as_ref(), cultivar_name.as_bytes().as_ref()], bump, space = 8 + Cultivar::INIT_SPACE)]
    pub cultivar: Account<'info,Cultivar>,   

    /// CHECK: Pda authority 
     #[account(seeds=[b"fruitmintauthority"], bump,)]
    pub fruit_mint_authority: UncheckedAccount<'info>,

    #[account(init, payer=payer , seeds=[b"fruitmint", 
    cultivar_name.as_bytes().as_ref()],bump, mint::decimals=9, mint::authority=fruit_mint_authority)] // different fruits 
    pub fruit_mint: Account<'info, Mint>, 
    
    #[account(init_if_needed, payer=payer, seeds=[b"fruitmarket", cultivar_name.as_bytes().as_ref()], bump, space =  8 + FruitMarket::INIT_SPACE)]
    pub fruit_market: Account<'info,FruitMarket>,

    #[account(init_if_needed, payer=payer, seeds=[b"marketentry", fruit_market.key().as_ref(),program_id.as_ref()],space =  8 + MarketEntry::INIT_SPACE,bump)]
    pub market_entry: Account<'info,MarketEntry>,

    // #[account(init_if_needed, payer=payer,seeds=[b"marketentry", fruit_market.key().as_ref(),program_id.as_ref()], space =  8 + MarketEntry::INIT_SPACE, bump)]
    // pub current_top_market_entry: Box<Account<'info,MarketEntry>>,

    pub token_program: Program<'info,Token>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(cultivar_name:String)]
pub struct PlantTree <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(seeds=[b"farmer", payer.key().as_ref()], bump,)]
    pub farmer: Box<Account<'info,Farmer>>,     
    #[account(seeds=[b"farm"], bump)]
    pub farm: Box<Account<'info,Farm>>,
    #[account(mut, seeds=[b"cultivarmeta",farm.key().as_ref()], bump,)]
    pub cultivar_meta: Account<'info, CultivarMeta>,
    #[account(mut, seeds=[b"treesmeta",farm.key().as_ref()], bump,)]
    pub trees_meta: Account<'info, TreesMeta>,
    #[account(mut,seeds=[b"cultivar", cultivar_meta.key().as_ref(), cultivar_name.as_bytes().as_ref()], bump,)]
    pub cultivar: Account<'info,Cultivar>,    
    #[account(init, payer=payer, seeds=[b"tree",trees_meta.key().as_ref(),farmer.key().as_ref()], bump, space = 8 + Tree::INIT_SPACE)]
    pub tree: Account<'info, Tree>,
    /// CHECK: Pda authority 
     #[account(seeds=[b"fruitmintauthority"], bump,)]
    pub fruit_mint_authority: UncheckedAccount<'info>,        
    #[account(seeds=[b"fruitmint", cultivar_name.as_bytes().as_ref()],bump, mint::decimals=9, mint::authority=fruit_mint_authority)] // different fruits 
    pub fruit_mint: Account<'info, Mint>, 
    #[account(init, seeds=[b"fruit", tree.key().as_ref()],bump, payer=payer, token::mint=fruit_mint, token::authority=farmer)]
    pub fruit_balance: Account<'info,TokenAccount>,
    #[account(seeds=[b"landmeta", farm.key().as_ref()], bump, )]
    pub land_meta: Account<'info,LandMeta>,
    #[account(mut,seeds=[b"landpiece",land_meta.key().as_ref(),farmer.key().as_ref()], bump,)]
    pub land_piece: Account<'info, LandPiece>,
    #[account(init, payer=payer, seeds=[b"nutrientbalance",land_piece.key().as_ref(),tree.key().as_ref()], bump, space = 8 + LandPiece::INIT_SPACE)]
    pub input_balance: Account<'info, InputBalance>,
    #[account(init ,payer=payer, seeds=[b"requirednutrients", tree.key().as_ref()], bump, space= 8 + RequiredNutrients::INIT_SPACE)]
    pub required_nutrients: Box<Account<'info,RequiredNutrients>>,
    pub token_program: Program<'info,Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct BuyLand <'info> { 
    #[account(mut)]
    pub payer: Signer<'info>,      
    #[account(seeds=[b"farm"], bump)]
    pub farm: Account<'info,Farm>, 
    #[account(seeds=[b"farmer", payer.key().as_ref()], bump)]
    pub farmer: Account<'info,Farmer>, 
    #[account(mut,seeds=[b"landmeta", farm.key().as_ref()], bump, )]
    pub land_meta: Account<'info,LandMeta>,
    #[account(init, payer=payer, seeds=[b"landpiece",land_meta.key().as_ref(),farmer.key().as_ref()], bump, space = 8 + LandPiece::INIT_SPACE)]
    pub land_piece: Account<'info, LandPiece>,
    #[account(mut, seeds=[b"carbonvault"], bump,)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>
}


#[derive(Accounts)]
pub struct InitializeFarmer <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer=payer, seeds=[b"farmer", payer.key().as_ref()], bump, space = 8 + Farmer::INIT_SPACE)]
    pub farmer: Account<'info,Farmer>, 
    pub token_program: Program<'info,Token>,
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


pub const SLOTS_PER_SECOND: u64=DEFAULT_TICKS_PER_SECOND /DEFAULT_TICKS_PER_SLOT ;

pub const SLOTS_PER_DAY: u64 =SLOTS_PER_SECOND*SECONDS_PER_DAY ;

pub const SLOTS_PER_YEAR: u64 = SLOTS_PER_DAY*365;

pub const WIDTH_PER_YEAR: u64 = 25000 ; // 25000micrometers 
pub const HEIGHT_PER_YEAR: u64 = 500000;
pub const HEIGHT_PER_SLOT: u64  =  HEIGHT_PER_YEAR / SLOTS_PER_YEAR ;
pub const WIDTH_PER_SLOT: u64  =  WIDTH_PER_YEAR / SLOTS_PER_YEAR ;

pub const ROOT_AREA_GROWTH_RATE:u64  = 1 ;
pub const LEAF_AREA_GROWTH_RATE: u64 = 1 ;

pub const RATE_OF_NUTRIENT_UPTAKE: u64 = 1; 
pub const RATE_OF_WATER_UPTAKE: u64 = 2; 
pub const RATE_OF_FRUIT_INCREMENT: u64 = 1 ;


impl Tree {

pub fn update_size(&mut self, percent_nitrogen_intake: u64 , percent_phosphorus_intake: u64, percent_potassium_intake: u64, percent_water_intake: u64,recent_check_time: u64 ) -> Result<()>  {
    //calcuate food consumption
    // return food consumption based on tree attributes     
    let period =  recent_check_time - self.last_check_time ;
    let reduction = percent_nitrogen_intake* percent_phosphorus_intake*percent_potassium_intake *percent_water_intake ;

    let height =  self.height + period * HEIGHT_PER_SLOT * reduction /(100 * 100* 100* 100) ;
    let width =  self.girth + period * WIDTH_PER_SLOT * reduction /(100 * 100* 100 * 100) ;

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
    self.age += period;

    Ok(())
}

pub fn set_last_check_time(&mut self) -> Result<()>{
    self.last_check_time = Clock::get()?.slot ;  //since i am updating using the time from the required_nutrients acount there is no need for this at the moment 
    Ok(())
}

pub fn grow_leaf_area(&mut self, percent_nitrogen_intake: u64, percent_water_intake: u64, recent_check_time: u64) -> Result<()> {    
 //use some nitrogen 
   let period =  recent_check_time - self.last_check_time ;
  self.leaf_area += LEAF_AREA_GROWTH_RATE * period * percent_nitrogen_intake* percent_water_intake/ 100 * 100 ;
  Ok(())
}

pub fn grow_root_area(&mut self, percent_phosphorus_intake: u64, percent_water_intake: u64, recent_check_time: u64) -> Result<()> { 
    let inc = 25400.0 ;
    let ft = 304800.0 * 1.5 ;
    let root_area = ((self.girth as f64 / inc) * ft) as u64 ;
    self.root_area += root_area * percent_phosphorus_intake * percent_water_intake/ 100 * 100 ;
    Ok(())
}

pub fn grow_expected_fruit_count(&mut self, percent_potassium_intake: u64, percent_water_intake: u64, recent_check_time: u64) -> Result<()> {  
 //use some nitrogen 
  let period =  recent_check_time - self.last_check_time ;
  let too_young = SLOTS_PER_YEAR ;
  let young  = SLOTS_PER_YEAR* 10 ;
  let mut age_factor =  0 ; 
  if self.age > too_young && self.age < young {   
     age_factor = 50 ;
  };
  if self.age > young {
     age_factor = 100 ;
  }; 

  msg!("age factor :{}", age_factor) ;
  self.expected_fruit_count +=  RATE_OF_FRUIT_INCREMENT * period * percent_potassium_intake * percent_water_intake * age_factor / 100 *100 * 100  ;
  Ok(())
}

pub fn is_harvest_season(&mut self) -> Result<bool>{
    if Clock::get()?.slot >= self.next_fruit_maturaturation_time{
        return Ok(true)
    }
     Ok(false)
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


}


impl InputBalance {

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

    pub fn percentage_intake(&self ,root_area: u64, balance: u64 , last_checked_time: u64, age: u64, water: bool)-> Result<(u64,u64)> {
         // , balance: {}
        msg!("root area: {}, last_checked_time: {} , age: {} , water: {} ",root_area , last_checked_time, age, water );
        let required:u64 ;

        let too_young = SLOTS_PER_YEAR ;
        let young  = SLOTS_PER_YEAR* 10 ;
        let age_factor = match age {
        x if x < too_young => {
              100
            },
        x  if x < young => {
               70
           },
        _  => {50}
        } ;

    // let root_factor = match root_area {


    // }
        required =  match water {
            true => { Self::required_water_uptake(root_area,last_checked_time, age_factor)?},
            false =>{ Self::required_uptake(root_area, last_checked_time, age_factor)?}  
        } ;
     
        msg!("the required: {} , ",required) ;
        msg!("Howmay slots per second: {} , ",SLOTS_PER_SECOND);

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


// #[error_code]
// pub enum TreeError{

// }




// Calculating critical root radius 
//https://www.acompletetreecare.com/blog/how-to-measure-a-trees-critical-root-zone/
// The general rule of thumb is that for every inch of tree trunk, the radius increases by 1.5 feet. 

//1inch => 25400 micrometers 

//1 foot => 304800 micromers