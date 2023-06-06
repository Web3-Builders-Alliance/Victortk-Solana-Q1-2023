use anchor_lang::prelude::*;
use anchor_spl::{
    mint, token::{self,Mint,TokenAccount,Token, },
    associated_token::{AssociatedToken,Create,},   
};
use anchor_lang::solana_program::{pubkey,} ;
use solana_program::{native_token::LAMPORTS_PER_SOL};
use anchor_lang::system_program;
use solana_program::clock::{DEFAULT_TICKS_PER_SLOT,DEFAULT_TICKS_PER_SECOND, SECONDS_PER_DAY} ;

// use farmer::cpi::accounts::InitializeFarmer;
// use tree::cpi::accounts::InitializeFarmer;
// use farmer::program::Farmer as FarmerProgram;
use farm::program::Farm as FarmProgram;
use tree::program::Tree as TreeProgram;
// use farmer::{self,Farmer} ;
use farm::{self,Farm,TreesMeta,CultivarMeta} ;
use tree::{self,Cultivar,Tree} ;

declare_id!("9CkBCJW4mrYE7qpAgnnmWggeCYtyyKnfzJNZXY2mhpUU");

#[program]
pub mod fruit_market {
    use super::*;

     pub fn initialize_market(ctx:Context<InitializeFruitMarket>) ->Result<()>{
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
        Ok(())
     }


     pub fn create_market(ctx:Context<CreateMarket>,cultivar_name:String) ->Result<()>{
        // transfer tokens quantity  from tree to the market token account 
      
        // // fruit_mint_authority
        // let fruit_mint_authority = &mut ctx.accounts.fruit_mint_authority ;
        // // fruit_mint
        //  let fruit_mint = &mut ctx.accounts.fruit_mint ;
        // // fruit_vault
        //  let entry_fruit_balance = &mut ctx.accounts.entry_fruit_balance ;
        // // fruit_balance
        //  let fruit_balance = &mut *ctx.accounts.fruit_balance ;
        // //  let farmer = &mut ctx.accounts.farmer ;
         let payer = &mut ctx.accounts.payer.key() ;     

         
         //increment the market volume info 
         let market = &mut *ctx.accounts.fruit_market ;

         msg!("market b4 was  :{:?} ", market.top_maker);
         let market_entry  = &mut *ctx.accounts.market_entry ;

         let current_top_market_entry  = &mut **ctx.accounts.current_top_market_entry ;
         
        //   maker: Pubkey ,
        //   count: u64 ,
        //   next_maker: Option<Pubkey>,
        // msg!("the maker is : ======>> {}", payer ) ;

        market_entry.maker = *payer ;
        // market_entry.count = amount ;

        market_entry.next_maker = Some(current_top_market_entry.maker) ;       
        
        market.top_maker = Some(market_entry.maker);  
        market.total_fruit_balance += market_entry.count ;

        // let bump = *ctx.bumps.get("farmer").unwrap() ;
        // let seeds = &["farmer".as_bytes(), payer.as_ref(), &[bump]] ;
       
        // let k =  pubkey::Pubkey::find_program_address(&["farmer".as_bytes(), payer.as_ref()], ctx.program_id);

    

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

        let market_authority  = &mut ctx.accounts.market_authority ;

        //   maker: Pubkey ,
        //   count: u64 ,
        //   next_maker: Option<Pubkey>,

        let bump  = *ctx.bumps.get("market_authority").unwrap();
        let seeds=&["marketauthority".as_bytes(), &[bump]] ;
		// topEntryFruitBalance,  
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info().clone(), 
                token::Transfer{
                  from: top_entry_fruit_balance.to_account_info().clone(), 
                  to: seeds_balance.to_account_info().clone(),
                  authority: market_authority.to_account_info().clone(), 
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

    //  /// CHECK: famer 
    // pub farmer: UncheckedAccount<'info,>,

    #[account(seeds=[b"farm"], bump, seeds::program=farm_program)]
    pub farm: Box<Account<'info,Farm>>,

    #[account(mut,seeds=[b"cultivarmeta",
    farm.key().as_ref()], bump,seeds::program=farm_program)]
    pub cultivar_meta: Box<Account<'info, CultivarMeta>>,

    #[account(mut,seeds=[b"cultivar", cultivar_meta.key().as_ref(), cultivar_name.as_bytes().as_ref()], bump, seeds::program=tree_program)]
    pub cultivar: Box<Account<'info,Cultivar>>,   
    
    /// CHECK: fruit mint authourity pda 
    #[account(mut, seeds=[b"fruitmintauthority"], bump,seeds::program=tree_program)]
    pub fruit_mint_authority: UncheckedAccount<'info>,

     #[account(mut, seeds=[b"marketauthority"], bump,)]
    /// CHECK: fruit mint authourity pda  
    pub market_authority: UncheckedAccount<'info>,


    #[account(mut,seeds=[b"fruitmint",cultivar_name.as_bytes().as_ref()], bump, mint::decimals=9, mint::authority=fruit_mint_authority,seeds::program=tree_program)]
    pub fruit_mint: Account<'info, Mint>,

    // #[account(init_if_needed, payer=payer , seeds=[b"seedvault",farmer.key().as_ref()], bump,space=8 + SeedVault::INIT_SPACE )] //token::authority=program. does that happen by default 
    // pub seed_vault: Box<Account<'info,SeedVault>>, 

    /// CHECK: It is used to derive other accounts which are checked
    #[account(seeds=[b"seedsauthority", payer.key().as_ref()], bump,seeds::program=tree_program)]
    pub seeds_authority: UncheckedAccount<'info>, 

    #[account(mut ,seeds=[b"seedsbalance", seeds_authority.key().as_ref(),cultivar_name.as_bytes().as_ref()],bump,token::mint=fruit_mint, token::authority=seeds_authority,seeds::program=tree_program)]
    pub seeds_balance: Box<Account<'info,TokenAccount>>,
    
    #[account(mut,seeds=[b"fruitmarket", cultivar_name.as_bytes().as_ref()], bump)]
    pub fruit_market: Account<'info,FruitMarket>,

    #[account(mut,seeds=[b"marketentry", fruit_market.key().as_ref(),fruit_market.top_maker.unwrap().as_ref()], bump)]
    pub current_top_market_entry: Account<'info,MarketEntry>,

    #[account(mut,seeds=[b"fruit",current_top_market_entry.key().as_ref()], bump, token::mint=fruit_mint,token::authority=market_authority)]
    pub top_entry_fruit_balance: Account<'info,TokenAccount>,
    
    pub tree_program: Program<'info,TreeProgram>,
    pub farm_program: Program<'info,FarmProgram>,
    pub system_program: Program<'info,System>,
    pub token_program: Program<'info,Token>,
  
}


#[derive(Accounts)]

pub struct InitializeFruitMarket<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,    
    #[account(seeds=[b"farm"], bump, seeds::program =farm_program)]
    pub farm: Box<Account<'info,Farm>>,
     /// CHECK: famer 
    pub farmer: UncheckedAccount<'info,>,

    #[account( seeds=[b"treesmeta",farm.key().as_ref()], bump,seeds::program=farm_program )]
    pub trees_meta: Box<Account<'info, TreesMeta>>,

     #[account(seeds=[b"tree",trees_meta.key().as_ref(),farmer.key().as_ref(),tree.cultivar_name.as_bytes().as_ref()], bump, seeds::program=tree_program )]
    pub tree: Box<Account<'info, Tree>>,

     #[account(init, payer=payer, seeds=[b"fruitmarket", tree.cultivar_name.as_bytes().as_ref()], bump , space =  8 + FruitMarket::INIT_SPACE)]
    pub fruit_market: Account<'info,FruitMarket>,

    #[account(init_if_needed, payer=payer, seeds=[b"marketentry", fruit_market.key().as_ref(),program_id.as_ref()],space =  8 + MarketEntry::INIT_SPACE,bump)]
    pub market_entry: Account<'info,MarketEntry>,

    pub system_program: Program<'info,System>,
    pub tree_program: Program<'info,TreeProgram>,
    pub farm_program: Program<'info,FarmProgram>,
    pub token_program: Program<'info,Token>,
}

#[derive(Accounts)]
#[instruction(cultivar_name:String)]
pub struct CreateMarket<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,    

    #[account(mut, seeds=[b"fruitmarket", cultivar_name.as_bytes()], bump)]
    pub fruit_market: Account<'info,FruitMarket>,
      /// CHECK: fruit mint authourity pda  
    pub fruit_mint_authority: UncheckedAccount<'info>,

    #[account(mut, seeds=[b"marketauthority"], bump,)]
    /// CHECK: fruit mint authourity pda  
    pub market_authority: UncheckedAccount<'info>,

    #[account(mint::decimals=9, mint::authority=fruit_mint_authority,)] // different fruits 
    pub fruit_mint: Box<Account<'info, Mint>>,

    #[account(init, payer=payer, seeds=[b"marketentry", fruit_market.key().as_ref(),payer.key().as_ref()],space =  8 + MarketEntry::INIT_SPACE,bump,)]
    pub market_entry: Account<'info,MarketEntry>,
    
    #[account(seeds=[b"marketentry", fruit_market.key().as_ref(),fruit_market.top_maker.unwrap().as_ref()], bump,)]
    pub current_top_market_entry: Box<Account<'info,MarketEntry>>,

    #[account(init, payer=payer, seeds=[b"fruit",market_entry.key().as_ref(),], bump, token::mint=fruit_mint, token::authority=market_authority)]
    pub entry_fruit_balance: Account<'info,TokenAccount>, 
      
    pub system_program: Program<'info,System>,
    pub token_program: Program<'info,Token>,
}



#[account]
#[derive(Default)]
#[derive(InitSpace)]
pub struct MarketEntry {
  pub maker: Pubkey ,
  count: u64 ,
  pub next_maker: Option<Pubkey>,
}

#[account]
#[derive(Default)]
#[derive(InitSpace)]
pub struct FruitMarket {
   pub top_maker: Option<Pubkey> ,
   pub bottom_maker: Option<Pubkey> ,
   total_fruit_balance: u64 ,
   price: u64 ,
}
