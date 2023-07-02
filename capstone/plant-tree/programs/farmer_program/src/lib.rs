use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey;
use anchor_spl::{
    associated_token::{AssociatedToken, Create},
    mint,
    token::{self, Mint, Token, TokenAccount},
};
use farm_program::cpi::accounts::UpdateFarm;
// use fruit_market::cpi::accounts::{ListFruit};
use tree_program::cpi::accounts::CreateTree;
// use solana_sdk::account_info::AccountInfo ;
use farm_program::program::FarmProgram;
use fruit_market_program::program::FruitMarketProgram;
use tree_program::program::TreeProgram;

use farm_program::{self, CultivarMeta, Farm, LandMeta, TreesMeta, Vault};
use fruit_market_program::{self, FruitMarket, MarketEntry};
use tree_program::{self, Cultivar, InputBalance, RequiredNutrients, Tree};

use anchor_lang::system_program;
use solana_program::clock::{DEFAULT_TICKS_PER_SECOND, DEFAULT_TICKS_PER_SLOT, SECONDS_PER_DAY};
use solana_program::native_token::LAMPORTS_PER_SOL;

declare_id!("3pEgxEH8RhxKtdx3qsvcmrZQUMxeyQisiiBAJ52FmtMx");

#[program]
pub mod farmer_program {
    use super::*;
    pub fn initialize_farmer(ctx: Context<InitializeFarmer>, user_name: String) -> Result<()> {
        let payer = &mut ctx.accounts.payer;
        let farmer = &mut ctx.accounts.farmer;
        farmer.name = user_name;
        farmer.address = payer.key().clone();
        farmer.land_count = 0;
        farmer.tree_count = 0;
        Ok(())
    }

    pub fn add_tree_count(ctx: Context<UpdateFarmer>) -> Result<()> {
        ctx.accounts.farmer.tree_count += 1;
        Ok(())
    }
    pub fn add_land_count(ctx: Context<UpdateFarmer>) -> Result<()> {
        ctx.accounts.farmer.land_count += 1;
        Ok(())
    }
    pub fn list_fruits(ctx: Context<ListFruits>, cultivar_name: String, amount: u64) -> Result<()> {
        let payer = &mut ctx.accounts.payer.to_account_info();
        let fruit_vault = &mut *ctx.accounts.fruit_vault;
        let farmer = &mut ctx.accounts.farmer;
        let entry_fruit_balance = &mut ctx.accounts.entry_fruit_balance;
        let bump = *ctx.bumps.get("farmer").unwrap();
        let seeds = &["farmer".as_bytes(), payer.key.as_ref(), &[bump]];
        // let k =  pubkey::Pubkey::find_program_address(&["farmer".as_bytes(), payer.as_ref()], ctx.program_id);
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info().clone(),
                token::Transfer {
                    from: fruit_vault.to_account_info().clone(),
                    to: entry_fruit_balance.to_account_info().clone(),
                    authority: farmer.to_account_info().clone(),
                },
                &[&seeds[..]],
            ),
            amount,
        )?;
        Ok(())
    }

    pub fn create_harvest_account(
        ctx: Context<CreateHarvestAccount>,
        cultivar_name: String,
    ) -> Result<()> {
        Ok(())
    }
    pub fn buy_land(ctx: Context<BuyLand>) -> Result<()> {
        let farmer = &mut ctx.accounts.farmer;
        let land_piece = &mut ctx.accounts.land_piece;
        let land_meta = &mut ctx.accounts.land_meta;
        let payer = &mut ctx.accounts.payer;
        let vault = &mut ctx.accounts.vault;
        ctx.accounts.farmer.land_count += 1;
        land_meta.land_piece_count += 1;
        land_piece.location = [land_meta.x_coord,land_meta.y_coord];
        land_meta.next_location()? ;
        land_piece.owner = ctx.accounts.farmer.to_account_info().key();
        land_piece.number = land_meta.land_piece_count;
        
        // transfer sol to vault
        let lamports = LAMPORTS_PER_SOL / 4;
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: payer.to_account_info(),
                    to: vault.to_account_info(),
                },
            ),
            lamports,
        )
    }

    pub fn plant(ctx: Context<Plant>) -> Result<()> {
        let farmer = &mut ctx.accounts.farmer;
        let tree = &mut ctx.accounts.tree;
        let land_piece = &mut ctx.accounts.land_piece;
        let slot = Clock::get()?.slot ;
        farmer.tree_count += 1;
        tree.land_number = land_piece.number;
        land_piece.is_planted = true;
        tree.location = land_piece.location ;
        tree.is_planted = true ;
        tree.planted_time = slot  ;
        
        Ok(())
    }

    pub fn close_farmer(ctx: Context<CloseFarmer>,) -> Result<()> {
        Ok(())
    }
    pub fn close_land(ctx: Context<CloseLand>,coordinates: [u8;2]) -> Result<()> {
        Ok(())
    }

    pub fn set_profile_image_uri(ctx: Context<SetUri>, uri: String ) -> Result<()>{
        ctx.accounts.farmer.profile_nft = uri ;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(uri: String)]
pub struct SetUri<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut,seeds=[b"farmer", payer.key().as_ref()], bump,)]
    pub farmer: Account<'info, Farmer>,
}

#[derive(Accounts)]
pub struct InitializeFarmer<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init_if_needed, payer=payer,seeds=[b"farmer", payer.key().as_ref()], bump, space = 8 + Farmer::INIT_SPACE)]
    pub farmer: Account<'info, Farmer>,
    // pub token_program: Program<'info,Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseFarmer<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut,close=payer,seeds=[b"farmer", payer.key().as_ref()], bump,)]
    pub farmer: Account<'info, Farmer>,
}

#[derive(Accounts)]
#[instruction(coordinates: [u8;2])]
pub struct CloseLand<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds=[b"farm"], bump,seeds::program=farm_program)]
    pub farm: Account<'info, Farm>,
    #[account(mut, seeds=[b"farmer", payer.key().as_ref(),], bump,)]
    pub farmer: Account<'info, Farmer>,
    #[account(mut,seeds=[b"landmeta", farm.key().as_ref()], bump,seeds::program=farm_program )]
    pub land_meta: Account<'info, LandMeta>,
    #[account(mut, close=payer, seeds=[b"landpiece",land_meta.key().as_ref(),farmer.key().as_ref(),&coordinates], bump,)]
    pub land_piece: Account<'info, LandPiece>,
    pub farm_program: Program<'info, FarmProgram>,
}

#[derive(Accounts)]
#[instruction(cultivar_name:String)]
pub struct CreateHarvestAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    ///CHECK: here were'e initializing a fruit vault    
    pub fruit_mint_authority: UncheckedAccount<'info>,
    #[account(mint::decimals=9, mint::authority=fruit_mint_authority,)] // different fruits
    pub fruit_mint: Account<'info, Mint>,
    #[account(mut,seeds=[b"farmer", payer.key().as_ref()], bump, )]
    pub farmer: Account<'info, Farmer>,
    #[account(init, seeds=[b"fruit", cultivar_name.as_bytes()],bump, payer=payer, token::mint=fruit_mint, token::authority=farmer)]
    pub fruit_vault: Box<Account<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(cultivar_name:String)]
pub struct ListFruits<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(seeds=[b"farm"], bump, seeds::program=farm_program)]
    pub farm: Box<Account<'info, Farm>>,

    #[account(mut, seeds=[b"farmer", payer.key().as_ref()], bump, )]
    pub farmer: Account<'info, Farmer>,

    // #[account(seeds=[b"fruitvaultauthority"], bump)]
    // /// CHECK: this is  a PDA authrity acc
    // pub fruit_vault_authority: UncheckedAccount<'info>,
    #[account(mut, seeds=[b"marketauthority"], bump,seeds::program=market_program,)]
    /// CHECK: fruit mint authourity pda  
    pub market_authority: UncheckedAccount<'info>,

    /// CHECK: fruit mint authourity pda  
    pub fruit_mint_authority: UncheckedAccount<'info>,

    #[account(mint::decimals=9, mint::authority=fruit_mint_authority,)] // different fruits
    pub fruit_mint: Box<Account<'info, Mint>>,

    #[account(mut, seeds=[b"fruit", cultivar_name.as_bytes()],bump, token::mint=fruit_mint, token::authority=farmer)]
    pub fruit_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut, seeds=[b"fruitmarket", cultivar_name.as_bytes()], bump,seeds::program=market_program,)]
    pub fruit_market: Account<'info, FruitMarket>,

    #[account(seeds=[b"marketentry", fruit_market.key().as_ref(),payer.key().as_ref()],bump,seeds::program=market_program,)]
    pub market_entry: Account<'info, MarketEntry>,

    #[account(seeds=[b"marketentry", fruit_market.key().as_ref(),fruit_market.top_maker.unwrap().as_ref()], bump,seeds::program=market_program)]
    pub current_top_market_entry: Box<Account<'info, MarketEntry>>,

    #[account(mut,seeds=[b"fruit",market_entry.key().as_ref(),], bump, token::mint=fruit_mint, token::authority=market_authority, seeds::program=market_program)]
    pub entry_fruit_balance: Account<'info, TokenAccount>, //entry you are making

    pub farm_program: Program<'info, FarmProgram>,
    pub market_program: Program<'info, FruitMarketProgram>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Plant<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,seeds=[b"farmer", payer.key().as_ref()], bump,)]
    pub farmer: Account<'info, Farmer>,

    #[account(mut,seeds=[b"farm"], bump, seeds::program=farm_program)]
    pub farm: Box<Account<'info, Farm>>,

    #[account(mut, seeds=[b"cultivarmeta",farm.key().as_ref()], bump, seeds::program=farm_program)]
    pub cultivar_meta: Account<'info, CultivarMeta>,

    #[account(mut, seeds=[b"treesmeta",farm.key().as_ref()], bump,seeds::program=farm_program)]
    pub trees_meta: Account<'info, TreesMeta>,

    #[account(mut,seeds=[b"cultivar", cultivar_meta.key().as_ref(), cultivar.name.as_bytes().as_ref()], bump,seeds::program=tree_program)]
    pub cultivar: Account<'info, Cultivar>,

    #[account(mut,seeds=[b"tree",trees_meta.key().as_ref(),farmer.key().as_ref(),cultivar.name.as_bytes().as_ref(),tree.created_date.as_bytes()], bump, seeds::program=tree_program)]
    pub tree: Account<'info, Tree>,

    #[account(mut,seeds=[b"landmeta", farm.key().as_ref()], bump, seeds::program=farm_program)]
    pub land_meta: Account<'info, LandMeta>,

    #[account(mut,seeds=[b"landpiece",land_meta.key().as_ref(),farmer.key().as_ref(),&[land_meta.x_coord,land_meta.y_coord]], bump,)]
    pub land_piece: Account<'info, LandPiece>,
    pub farm_program: Program<'info, FarmProgram>,
    pub tree_program: Program<'info, TreeProgram>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateFarmer<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut,seeds=[b"farmer", payer.key().as_ref()], bump,)]
    pub farmer: Account<'info, Farmer>,
}

#[derive(Accounts)]
pub struct BuyLand<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds=[b"farm"], bump,seeds::program=farm_program)]
    pub farm: Account<'info, Farm>,
    #[account(mut, seeds=[b"farmer", payer.key().as_ref(),], bump,)]
    pub farmer: Account<'info, Farmer>,

    #[account(mut,seeds=[b"landmeta", farm.key().as_ref()], bump,seeds::program=farm_program )]
    pub land_meta: Account<'info, LandMeta>,

    #[account(init, payer=payer, seeds=[b"landpiece",land_meta.key().as_ref(),farmer.key().as_ref(),&[land_meta.x_coord,land_meta.y_coord]], bump, space = 8 + LandPiece::INIT_SPACE)]
    pub land_piece: Account<'info, LandPiece>,

    #[account(mut, seeds=[b"carbonvault"], bump,seeds::program=farm_program)]
    pub vault: Account<'info, Vault>,
    pub farm_program: Program<'info, FarmProgram>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default, InitSpace)]
pub struct Farmer {
    #[max_len(30)]
    pub name: String,
    pub address: Pubkey,
    pub land_count: u64,
    pub tree_count: u64,
    #[max_len(150)]
    pub profile_nft: String ,
}

#[account]
#[derive(Default, InitSpace)]
pub struct LandPiece {
    pub owner: Pubkey,
    pub number: u64,
    pub is_planted: bool,
    pub location: [u8;2],
}

// #[error_code]
// pub enum FarmerError {
//     #[msg("MyAccount may only hold data below 100")]
//     DataTooLarge
// }
