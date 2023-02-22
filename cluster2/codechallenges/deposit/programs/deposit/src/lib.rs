//### anotations 
use anchor_lang::prelude::*;//### i wonder what comes with the prelude
//### it includes includes types from the solana_program, sysvars like clock
//### It includes all the anchor types like Account, macros like declare_id
//### it also includes attribute like account and all the anchor goodies enums ...
use anchor_spl::{ //anchor spl expor
    associated_token::AssociatedToken,
    token::{self, TokenAccount, Transfer,Mint, Token},
    metadata,
    dex,
};

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

    pub fn create_market(ctx:Context<CreateMarket>,bid: u64, ask: u64)->Result<()>{
      dex::initialize_market(
        CpiContext::new(
            dex::ID, 
        dex::InitializeMarket{
            
        }
        )
        
      Ok(())
    }
 
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


#[account]
pub struct Market {
    pub maker: Pubkey ,
    pub ask_price: u64,
    pub bid_price: u64, 
    pub bump: u8 ,
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
    pub coin_mint: AccountInfo<'info>,
    pub pc_mint: AccountInfo<'info>,
    pub coin_vault: AccountInfo<'info>,
    pub pc_vault: AccountInfo<'info>,
    pub bids: AccountInfo<'info>,
    pub asks: AccountInfo<'info>,
    pub req_q: AccountInfo<'info>,
    pub event_q: AccountInfo<'info>,
    pub token_program: Program<'info,Token>,
    pub system_program: Program<'info,System>,
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






