use anchor_lang::prelude::*;
declare_id!("8qCGCh9UYRicNDoZzzjkjLvidj1asvnuvYUJu5KJbCL9");

#[program]
pub mod deposit {
    use anchor_lang::system_program;
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,amount: u64 ) -> Result<()> {
        let account = &mut ctx.accounts.vault ;
        account.authority = *ctx.accounts.authority.to_account_info().key ;
        account.bump = *ctx.bumps.get("vault").unwrap() ;  
        msg!("{:?}",ctx.bumps) ;   
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
        seeds=[b"vault"], 
        bump,    
    )]
    pub vault: Account<'info,Vault>,
    pub system_program: Program<'info,System>
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds=[b"vault"], bump=vault.bump)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info,System>
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds=[b"vault"], bump=vault.bump, constraint = vault.authority == *authority.key)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info,System>
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut,close=authority, seeds=[b"vault"], bump=vault.bump, constraint = vault.authority == *authority.key)]
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




