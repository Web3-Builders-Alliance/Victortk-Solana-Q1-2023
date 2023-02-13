//### anotations 

use anchor_lang::prelude::*;//### i wonder what comes with the prelude 
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

    pub fn close (ctx: Context<Close>)-> Result<()> {
        let vault_balance = **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? ;
        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= vault_balance ;
        **ctx.accounts.authority.to_account_info().try_borrow_mut_lamports()? += vault_balance;
        Ok(())
    }
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

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds=[b"vault", authority.to_account_info().key.as_ref()], bump=vault.bump, constraint = vault.authority == *authority.key)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info,System>
}

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




