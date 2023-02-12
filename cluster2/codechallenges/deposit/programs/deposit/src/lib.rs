use anchor_lang::prelude::*;
declare_id!("8qCGCh9UYRicNDoZzzjkjLvidj1asvnuvYUJu5KJbCL9");

#[program]
pub mod deposit {
    use anchor_lang::system_program;
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,amount: u64 ) -> Result<()> {

        let account = &mut ctx.accounts.vault ;

        if account.is_initialized == true {
            return err!(VaultError::AlreadyInitialized)
        }
        account.owner = *ctx.accounts.owner.to_account_info().key ;
        account.balance = amount ;
        account.is_initialized = true ;
        account.bump = *ctx.bumps.get("vault").unwrap() ;  
        msg!("{:?}",ctx.bumps) ;   
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.owner.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(), 
                },
            ),
            amount,
        )?;
        Ok(())
    }

    pub fn deposit (ctx:Context<Deposit>, amount: u64) -> Result<()> {         
        //when i used this method it gave me an error, you cant modify an account you do not own 
        // **ctx.accounts.payer.to_account_info().try_borrow_mut_lamports()? -= amount ;
        // **ctx.accounts.pda.to_account_info().try_borrow_mut_lamports()? += amount ; 
        
        
        let account = &mut ctx.accounts.vault ;

        if account.is_initialized != true {
            return err!(VaultError::NotInitialized)
        }
        account.balance += amount ;
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.owner.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(), 
                },
            ),
            amount,
        )?;
        
        Ok(())
    }
}

#[account]
pub struct Vault{
    pub owner: Pubkey,
    pub balance: u64 ,
    pub is_initialized: bool , 
    pub bump: u8   
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        constraint = vault.is_initialized == false,
        payer=owner,
        space=8+8+32+1+1,         
        seeds=[b"vault"], 
        bump,
    
    )]
    pub vault: Account<'info,Vault>,
    pub system_program: Program<'info,System>
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, seeds=[b"vault"], bump=vault.bump)]
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




