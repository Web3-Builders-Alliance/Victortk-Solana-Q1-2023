use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

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
    pub fn create_cultivar(ctx: Context<CreateCultivar>, name: String , height: u64, width: u64) -> Result<()>{
        let cultivar_meta =  &mut ctx.accounts.cultivar_meta ;
         cultivar_meta.cultivars_count = cultivar_meta.cultivars_count + 1 ;
         let cultivar = &mut ctx.accounts.cultivar ;
             cultivar.count = 0 ;
             cultivar.name = name ;    
             cultivar.init_height= height ;
             cultivar.init_width= width ;
        Ok(())
    }

    pub fn initialize_farmer(ctx: Context<InitializeFarmer>, user_name: String, cultivar_name: String ) -> Result<()> {        
        let payer = &mut ctx.accounts.payer ; 
        let farmer = &mut ctx.accounts.farmer ;
        farmer.name = user_name ;
        farmer.address = payer.key().clone() ;
        farmer.land_count = 1 ;
        farmer.tree_count = 1 ;

        let land_meta = &mut ctx.accounts.land_meta ;
        land_meta.land_piece_count = land_meta.land_piece_count + 1 ; //to check for initialization
        
        let trees_meta = &mut ctx.accounts.trees_meta ;
        trees_meta.tree_count = trees_meta.tree_count + 1 ; //to check for initialization


        let cultivar = &mut ctx.accounts.cultivar ;
        cultivar.count = cultivar.count + 1 ;
        //scacity points cultivar.scarcity_points = 

        let land_piece = &mut ctx.accounts.land_piece ;
        land_piece.authority = payer.key().clone();
        land_piece.number = land_meta.land_piece_count ;
        land_piece.is_planted = true ;   

        
        let tree =  &mut ctx.accounts.tree ;
        tree.cultivar_name = cultivar_name ;
        tree.land_number = land_piece.number ;
        tree.height = cultivar.init_height;
        tree.girth = cultivar.init_width;
        tree.age = 0 ;
        let time =  Clock::get()?.slot ;
        tree.planted = time ;
        //ticks per second/ ticks per slot * 1 year in seconds should probably get the constants 
        tree.next_fruit_maturaturation_date =  time + 5 * 30 * 60 * 24 * 365 ; 


        let vault =  &mut ctx.accounts.vault ;
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
}


#[derive(Accounts)]
pub struct InitializeFarm <'info> {
     #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds=[b"farm"], bump)]
    pub farm: Account<'info,Farm>,

    #[account(init, payer=payer, seeds=[b"landmeta", farm.key().as_ref()], bump, space = 8 + LandMeta::INIT_SPACE)]
    pub land_meta: Account<'info,LandMeta>,

    #[account(seeds=[b"cultivarmeta",farm.key().as_ref()], bump)]
    pub cultivar_meta: Account<'info, CultivarMeta>,

    #[account(init, payer=payer, seeds=[b"treesmeta",farm.key().as_ref()], bump, space = 8 + TreesMeta::INIT_SPACE)]
    pub trees_meta: Account<'info, TreesMeta>,
    #[account(init, payer=payer, seeds=[b"carbonvault"], bump, space = 8 + Vault::INIT_SPACE)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}    


#[derive(Accounts)]
#[instruction(cultivar_name:String)]
pub struct CreateCultivar <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,    
    #[account(seeds=[b"farm"], bump)]
    pub farm: Account<'info,Farm>,
    #[account(seeds=[b"cultivarmeta",farm.key().as_ref()], bump,)]
    pub cultivar_meta: Account<'info, CultivarMeta>,
    #[account(init, payer=payer, seeds=[b"cultivar", cultivar_meta.key().as_ref(), cultivar_name.as_ref()], bump, space = 8 + Cultivar::INIT_SPACE)]
    pub cultivar: Account<'info,Cultivar>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(user_name:String, cultivar_name:String)]
pub struct InitializeFarmer <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, payer=payer, seeds=[b"farmer", payer.key().as_ref()], bump, space = 8 + Farmer::INIT_SPACE)]
    pub farmer: Account<'info,Farmer>, 

    #[account(seeds=[b"farm"], bump)]
    pub farm: Account<'info,Farm>,

    #[account(seeds=[b"landmeta", farm.key().as_ref()], bump, )]
    pub land_meta: Account<'info,LandMeta>,

    #[account(seeds=[b"cultivarmeta",farm.key().as_ref()], bump,)]
    pub cultivar_meta: Account<'info, CultivarMeta>,

    #[account( seeds=[b"treesmeta",farm.key().as_ref()], bump,)]
    pub trees_meta: Account<'info, TreesMeta>,

    #[account(init, payer=payer, seeds=[b"cultivar", cultivar_meta.key().as_ref(), cultivar_name.as_ref()], bump, space = 8 + Cultivar::INIT_SPACE)]
    pub cultivar: Account<'info,Cultivar>,


    #[account(init, payer=payer, seeds=[b"tree",trees_meta.key().as_ref(),farmer.key().as_ref()], bump, space = 8 + Tree::INIT_SPACE)]
    pub tree: Account<'info, Tree>,

    #[account(init, payer=payer, seeds=[b"landpiece",land_meta.key().as_ref(),farmer.key().as_ref()], bump, space = 8 + LandPiece::INIT_SPACE)]
    pub land_piece: Account<'info, LandPiece>,


    #[account(seeds=[b"carbonvault"], bump,)]
    pub vault: Account<'info, Vault>,
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
}

#[account]
#[derive(Default)]
#[derive(InitSpace)]
pub struct LandPiece {
    pub authority: Pubkey,
    pub number: u64,
    pub is_planted: bool,
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
    pub planted: u64,
    pub next_fruit_maturaturation_date: u64 ,
}




// #[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, )]
// #[derive(InitSpace)]
// pub enum TreeName{
//     Butternut = 1,
//     Mango,
//     Guava,
//     Eucalyptus 
// }

#[account]
#[derive(InitSpace)]
pub struct Vault {
 pub authority: Pubkey 
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