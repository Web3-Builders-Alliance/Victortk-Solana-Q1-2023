//### anotations 
use anchor_lang::prelude::*;//### i wonder what comes with the prelude
//### it includes includes types from the solana_program, sysvars like clock
//### It includes all the anchor types like Account, macros like declare_id
//### it also includes attribute like account and all the anchor goodies enums ...
use anchor_spl::{ //anchor spl expor
    associated_token::AssociatedToken,
    token::{self, TokenAccount, Transfer,Mint, Token,},
    dex::{self,Dex,serum_dex::{instruction::SelfTradeBehavior, matching::{Side,OrderType}}},
};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use borsh::{BorshDeserialize,BorshSerialize};
use std::num::NonZeroU64;



// impl BorshDeserialize for SelfTradeBehavior {

// }




// use solana_program::sysvar::rent;

declare_id!("8qCGCh9UYRicNDoZzzjkjLvidj1asvnuvYUJu5KJbCL9");
//#### i read once that declaire_id is like entrypoint
//#### also that the pubkey is used for validation, for example with PDA's??

#[program] //#### this program attribute expands into nore code which does?
pub mod deposit { //### declare deposit module
    use anchor_lang::system_program; //#### we include system_program from anchor
    use super::*; //### this just brings everything in the module above into scope
    
    //####initialize instruction function takes in initiaize context and an initial amount 
    pub fn initialize(ctx: Context<Initialize>,amount: u64 ) -> Result<()> {

        //### i should make sure initialization amount is abouve some minimum

        //#### here we  get a mutable reference to vault PDA account
        let account = &mut ctx.accounts.vault ;
        //## rules of borrowing say that there can only be one mutable reference to data at a time. 
        //### the fields on the vault account own their data soo 
        //### we use the * deref ( clone and to_owned seem to work aswell but they waste memory uncessarily i would suppose?)
        account.authority = *ctx.accounts.authority.to_account_info().key ;
        //### so here you use the name of the account in the Instructions Struct 
        //### instead of the seeds, hours were spent debuging that 
        account.bump = *ctx.bumps.get("vault").unwrap() ;  
        //### so whats cool is we can log stuff , and check the logs in .anchor/program_logs
        
        msg!("{:?}",ctx.bumps) ;   
        
        //### this is a little too nested 
        //### looks somewhat intimidating 
        //#### we have transfer function in the system_program module from anchor 
        system_program::transfer(
            CpiContext::new( //###CPI context should have the usual context stuff 
                //### like program_id of the program being CPI'd
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer { //####Transfer is a struct type that 
                    //#### is used to hold the required accounts for the context
                    from: ctx.accounts.authority.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(), 
                },
            ),
            amount, //####transfer takes a context and an amount  
        )?;
        Ok(())
    }

    pub fn create_market( ctx: Context<CreateMarket>,
        coin_lot_size: u64,
        pc_lot_size: u64,
        vault_signer_nonce: u64,
        pc_dust_threshold: u64,
    )->Result<()>{   
        dex::initialize_market(
            CpiContext::new( ctx.accounts.dex.to_account_info().clone()  , 
        dex::InitializeMarket{      
            market:ctx.accounts.market.to_account_info().clone(),  
            coin_mint:ctx.accounts.coin_mint.to_account_info().clone(),
            pc_mint:ctx.accounts.pc_mint.to_account_info().clone(),
            coin_vault:ctx.accounts.coin_vault.to_account_info().clone(),
            pc_vault:ctx.accounts.pc_vault.to_account_info().clone(),
            bids:ctx.accounts.bids.to_account_info().clone(),
            asks:ctx.accounts.asks.to_account_info().clone(),
            req_q: ctx.accounts.req_q.to_account_info().clone(),
            event_q:ctx.accounts.event_q.to_account_info().clone(),
            rent: ctx.accounts.rent.to_account_info(),
        }
        ),
        coin_lot_size,
        pc_lot_size,
        vault_signer_nonce,
        pc_dust_threshold,     
        )       
    
    }

    pub fn create_order (
    ctx: Context<NewOrder>,
    side: CurrentSide,
    limit_price: NonZeroU64,
    max_coin_qty: NonZeroU64,
    max_native_pc_qty_including_fees: NonZeroU64,
    self_trade_behavior: CurrentSelfTradeBehavior,
    order_type: CurrentOrderType,
    client_order_id: u64,
    limit: u16, ) -> Result<()>{

        let side: Side = side.into()  ;        

        dex::new_order_v3(
        CpiContext::new(
            ctx.accounts.dex.to_account_info().clone(),
            dex::NewOrderV3{
                market:ctx.accounts.market.to_account_info().clone(),
            open_orders: ctx.accounts.open_orders.to_account_info().clone(),
            request_queue: ctx.accounts.request_queue.to_account_info().clone() ,    
            event_queue: ctx.accounts.event_queue.to_account_info().clone() , 
            market_bids:ctx.accounts.market_asks.to_account_info().clone() , 
            market_asks: ctx.accounts.market_asks.to_account_info().clone() ,
            order_payer_token_account:ctx.accounts.order_payer_token_account.to_account_info().clone() , 
            open_orders_authority:ctx.accounts.open_orders.to_account_info().clone() , 
            coin_vault:ctx.accounts.coin_vault.to_account_info().clone() , 
            pc_vault:ctx.accounts.pc_vault.to_account_info().clone() , 
            token_program:ctx.accounts.token_program.to_account_info().clone() , 
            rent:ctx.accounts.rent.to_account_info().clone() , 
        } 
    ), 
    side,
        limit_price,
        max_coin_qty,
        max_native_pc_qty_including_fees,
        self_trade_behavior.into(),
        order_type.into(),
        client_order_id,
        limit)
    
    }

    // pub fn cancel_order(ctx:Context<CancelOrder>,side: Side,
    //     order_id: u128,) -> Result<()> {        
    //         dex::cancel_order_v2(
    //         CpiContext::new(
    //             ctx.accounts.dex.to_account_info().clone(),
    //         dex::CancelOrderV2{
    //              market:ctx.accounts.market.to_account_info().clone() ,
    //              market_bids: ctx.accounts.market_bids.to_account_info().clone() ,
    //              market_asks: ctx.accounts.market_asks.to_account_info().clone() ,
    //              open_orders: ctx.accounts.open_orders.to_account_info().clone() ,
    //              open_orders_authority: ctx.accounts.open_orders_authority.to_account_info().clone() ,
    //              event_queue: ctx.accounts.event_queue.to_account_info().clone() ,
    //         }
    //         ),
    //         side,
    //         order_id
    //     )
       
    // }
    
    pub fn deposit (ctx:Context<Deposit>, amount: u64) -> Result<()> {         
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.authority.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(), 
                },
            ),
            amount,
        )?;        
        Ok(())
    }

     pub fn withdraw (ctx:Context<Withdraw>, amount: u64) -> Result<()> {
        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= amount ;
        **ctx.accounts.authority.to_account_info().try_borrow_mut_lamports()? += amount ;
        Ok(()) 
    }

    pub fn deposit_tokens (ctx: Context<DepositToken>, amount: u64) -> Result<()> {
     //### Transfer is a struct type that is used to group the require accounts
     //### for the transfer function context 
     let accounts =  Transfer {
         from: ctx.accounts.deposit_associated_token_acc.to_account_info(),
         to: ctx.accounts.pda_associated_token_acc.to_account_info() ,
        authority: ctx.accounts.deposit_authority.to_account_info(),
    };
     //#### a context requires 2 things a program_id and accounts 
     let context = CpiContext::new(
         ctx.accounts.token_program.to_account_info(), accounts
     ) ;
     
    //### the transfer instruction is being called via CPI and it takes in a 
    //### a context and an amount 
    token::transfer(context, amount)

    }
    
    pub fn withdraw_tokens(ctx: Context<WithdrawToken>, amount: u64) -> Result<()>{
            token::transfer (
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.pda_associated_token_acc.to_account_info(),
                        to: ctx.accounts.withdraw_authority.to_account_info(),
                        authority: ctx.accounts.pda_authority.to_account_info()
                    },                    
                ),
                amount
            )
    }

    pub fn close (ctx: Context<Close>)-> Result<()> {
        let vault_balance = **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? ;
        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= vault_balance ;
        **ctx.accounts.authority.to_account_info().try_borrow_mut_lamports()? += vault_balance;
        Ok(())
    }
}



////Accounts that we need 
////1 --> Signer 
///2 ---> Vault to hold the market marker tokens 
 /// 3 ---> Token Mint A 
 /// 4 ---> Token Mint B
 /// 5 ---> PDA to own vault 
 /// 6 ----> signer token A associated account 
 /// 7 ----> signer token B associated account 
 /// 
 /// 
 /// pub struct InitializeMarket<'info> {
   // pub market: AccountInfo<'info>,
    // pub coin_mint: AccountInfo<'info>,
    // pub pc_mint: AccountInfo<'info>,
    // pub coin_vault: AccountInfo<'info>,
    // pub pc_vault: AccountInfo<'info>,
    // pub bids: AccountInfo<'info>,
    // pub asks: AccountInfo<'info>,
    // pub req_q: AccountInfo<'info>,
    // pub event_q: AccountInfo<'info>,
    // pub rent: AccountInfo<'info>,
    // }
      
    #[derive(Accounts)]
    pub struct CreateMarket<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: serum market
    pub market: UncheckedAccount<'info>,
    #[account(mut)]
    pub coin_mint: Account<'info, Mint>,
    #[account(mut)]
    pub pc_mint: Account<'info, Mint>,
    pub coin_vault: AccountInfo<'info>,
    pub pc_vault: AccountInfo<'info>,
    pub bids: AccountInfo<'info>,
    pub asks: AccountInfo<'info>,
    pub req_q: AccountInfo<'info>,
    pub event_q: AccountInfo<'info>,
    pub token_program: Program<'info,Token>,
    pub dex: Program<'info,Dex>,
    pub system_program: Program<'info,System>,
    /// CHECK: rent account 
    pub rent: UncheckedAccount<'info>,
    
}

#[derive(Accounts)]
pub struct NewOrder<'info> {
    pub market: AccountInfo<'info>,
    pub open_orders: AccountInfo<'info>,
    pub request_queue: AccountInfo<'info>,
    pub event_queue: AccountInfo<'info>,
    pub market_bids: AccountInfo<'info>,
    pub market_asks: AccountInfo<'info>,
    // Token account where funds are transferred from for the order. If
    // posting a bid market A/B, then this is the SPL token account for B.
    pub order_payer_token_account: AccountInfo<'info>,
    pub open_orders_authority: AccountInfo<'info>,
    // Also known as the "base" currency. For a given A/B market,
    // this is the vault for the A mint.
    pub coin_vault: AccountInfo<'info>,
    // Also known as the "quote" currency. For a given A/B market,
    // this is the vault for the B mint.
    pub pc_vault: AccountInfo<'info>,
    pub token_program: Program<'info,Token >,
    pub rent: Sysvar<'info, Rent>,
    pub dex: Program<'info,Dex>,
}


#[derive(Accounts)]
pub struct CancelOrder<'info> {
    pub market: AccountInfo<'info>,
    pub market_bids: AccountInfo<'info>,
    pub market_asks: AccountInfo<'info>,
    pub open_orders: AccountInfo<'info>,
    pub open_orders_authority: AccountInfo<'info>,
    pub event_queue: AccountInfo<'info>,
    pub dex: Program<'info,Dex>,
}


#[account]
pub struct Vault{
    pub authority: Pubkey,
    pub bump: u8   
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, 
        payer=authority,
        space=8+8+32+1+1,         
        seeds=[b"vault", authority.to_account_info().key.as_ref()], 
        bump,    
    )]
    pub vault: Account<'info,Vault>,
    pub system_program: Program<'info,System>
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds=[b"vault", authority.to_account_info().key.as_ref()], bump=vault.bump)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info,System>
}


#[derive(
    Eq, PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive, Debug, BorshDeserialize,BorshSerialize
)]
#[repr(u8)]
pub enum CurrentSide {
    Bid = 0,
    Ask = 1,
}

impl From<CurrentSide> for Side {
    fn from(side: CurrentSide) -> Side {
          match side {
            CurrentSide::Bid => Side::Bid,
            CurrentSide::Ask => Side::Ask            
          }
    }
}


#[derive(
    Eq, PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive, Debug, BorshDeserialize,BorshSerialize
)]

#[repr(u8)]
pub enum CurrentOrderType {
    Limit = 0,
    ImmediateOrCancel = 1,
    PostOnly = 2,
}

impl From<CurrentOrderType> for OrderType {
    fn from(order_type: CurrentOrderType) -> OrderType {
        match order_type {
            CurrentOrderType::Limit => OrderType::Limit,
            CurrentOrderType::ImmediateOrCancel => OrderType::ImmediateOrCancel,
            CurrentOrderType::PostOnly => OrderType::PostOnly
        }

    }
}

#[derive(
    PartialEq, Eq, Copy, Clone, Debug, TryFromPrimitive, IntoPrimitive, BorshDeserialize,BorshSerialize
)]
#[repr(u8)]
pub enum CurrentSelfTradeBehavior {
    DecrementTake = 0,
    CancelProvide = 1,
    AbortTransaction = 2,
}
impl From <CurrentSelfTradeBehavior> for SelfTradeBehavior {
    fn from(self_trade_behaviour: CurrentSelfTradeBehavior) -> SelfTradeBehavior {
        match self_trade_behaviour {
            CurrentSelfTradeBehavior::DecrementTake => SelfTradeBehavior::DecrementTake ,
            CurrentSelfTradeBehavior::CancelProvide => SelfTradeBehavior::CancelProvide,
            CurrentSelfTradeBehavior::AbortTransaction => SelfTradeBehavior::AbortTransaction,
        }
    }
}





//### marks this as an Instruction struct 
// #[derive(Accounts)]
// pub struct DepositToken<'info> { //#### DepositToken capitalize everyfirst word rust struct naming convention 
//     //#### 'info is a lifetime parameter which is equal to the lifetime of the longest of all the Account types that use it??

//  #[account(mut)] //### should the token mint be mutable?
//  pub token_mint: Account<'info, Mint> ,
//  #[account(mut)] //### we need this account to sign for the transaction and to sign the token transfer function 
//  pub deposit_authority: Signer<'info> ,
//  #[account( 
//     init_if_needed, //###we need a way to determine if we already have a associated token account
//     associated_token::mint = token_mint,//### here we tell anchor that if it ends up creating an associated token account to use token_mint account as the token "type", if already initialized to confirm that the token mint of the passed account matches that of token_mint
//     payer = deposit_authority,//### if init payer will be deposit_authority
//     associated_token::authority = pda_authority, //###if anchor initializes the account we tell it to make pda_authority the authority of the account 
//  )]
//  //###TokenAccount is a struct type that represents the structure of a token account, here we tell anchor to deserialize it into that type  
//  pub pda_associated_token_acc: Account<'info, TokenAccount>,
 
//  #[account(
//     //###We need seeds that make this account unique per user,
//     //###and unique per token mint/ which is token type  
//     seeds=[b"tokenvault",deposit_authority.to_account_info().key.as_ref(),token_mint.to_account_info().key.as_ref()],bump)]
//     //### we use seeds and bump to make sure that the right address is passed
//     /// CHECK: We are not using this inside the program and its a pda
//      pub pda_authority: UncheckedAccount<'info>,
//  //### for the depositors' ATA its supposed to already be initialize
//  //### under no circumstance is it allowed to no be
//  #[account(mut,
//      //### we need to make sure the account has the correct mint type 
//     associated_token::mint = token_mint,
//     //### we need to make sure the authority is deposit authority because t is supposed to sign for the transaction
//     associated_token::authority = deposit_authority,
//  )]
//  pub deposit_associated_token_acc: Account<'info, TokenAccount>,
//  //### the associated token program ATP is required when creating a new token account of a certain mint kind???
//  pub associated_token_program: Program<'info, AssociatedToken>,
//  //###the token program is required since its the owner of all token accounts, for creation and transfering the tokens 
//  pub token_program: Program<'info,Token>,
//  //### creating accounts always requires system_program???
//  pub system_program: Program<'info,System> 
// }

#[derive(Accounts)]
pub struct DepositToken<'info> {    
 #[account(mut)] 
 pub token_mint: Account<'info, Mint> ,
 #[account(mut)] 
 pub deposit_authority: Signer<'info> , 
 #[account( init_if_needed, associated_token::mint = token_mint, payer = deposit_authority,associated_token::authority = pda_authority,
 )] 
 pub pda_associated_token_acc: Account<'info, TokenAccount>,
 /// CHECK: PDA ACC
 #[account(    
    seeds=[b"tokenvault",deposit_authority.to_account_info().key.as_ref(),token_mint.to_account_info().key.as_ref()],bump)]   
 pub pda_authority: UncheckedAccount<'info>,

 #[account(mut,     
    associated_token::mint = token_mint,   
    associated_token::authority = deposit_authority,
 )]
 pub deposit_associated_token_acc: Account<'info, TokenAccount>, 
 pub associated_token_program: Program<'info, AssociatedToken>, 
 pub token_program: Program<'info,Token>, 
 pub system_program: Program<'info,System> 
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds=[b"vault", authority.to_account_info().key.as_ref()], bump=vault.bump, constraint = vault.authority == *authority.key)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info,System>
}

#[derive(Accounts)]
pub struct WithdrawToken<'info> {    
 #[account(mut)] 
 pub token_mint: Account<'info, Mint> ,
 #[account(mut)] 
 pub withdraw_authority: Signer<'info> ,
 #[account(associated_token::mint = token_mint,associated_token::authority = pda_authority,
 )] 
 pub pda_associated_token_acc: Account<'info, TokenAccount>,
 /// CHECK: pda account not using any data
 #[account(    
    seeds=[b"tokenvault",withdraw_authority.to_account_info().key.as_ref(),token_mint.to_account_info().key.as_ref()],bump)]   
 pub pda_authority: UncheckedAccount<'info>,
 #[account(     
    associated_token::mint = token_mint,   
    associated_token::authority = withdraw_authority,
 )]
 pub withdrawer_associated_token_acc: Account<'info, TokenAccount>,  
 pub token_program: Program<'info,Token>,
}

// #[derive(Accounts)]
// pub struct LimitOrder<'info>{


// }

// #[account()]
// pub struct Limit<'info> {
    
// }

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut,close=authority, seeds=[b"vault", authority.to_account_info().key.as_ref()], bump=vault.bump, constraint = vault.authority == *authority.key)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info,System>
}

#[error_code]
pub enum VaultError {
    #[msg("Vault already initialized")]
    AlreadyInitialized,
    #[msg("Vault not initialized")]
    NotInitialized
}






