use anchor_lang::prelude::*;
declare_id!("8qCGCh9UYRicNDoZzzjkjLvidj1asvnuvYUJu5KJbCL9");

#[program]
pub mod deposit {
    use anchor_lang::system_program;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {

        let account = &mut ctx.accounts.pda ;
        // account.owner = *ctx.accounts.payer.to_account_info().key ;
        account.amount = amount ;
        // account.bump = *ctx.bumps.get("depositsss").unwrap() ;

        // **ctx.accounts.payer.to_account_info().try_borrow_mut_lamports()? -= amount ;
        // **ctx.accounts.pda.to_account_info().try_borrow_mut_lamports()? += amount ;

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.pda.to_account_info(), 
                },
            ),
            amount,
        )?;

        Ok(())
    }
}

#[account]
pub struct Deposit{
    pub owner: Pubkey,
    pub amount: u64 , 
    pub bump: u8   
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: SystemAccount<'info>,
    #[account(init,payer=payer,space=8+8+32+1, seeds=[b"depositssss"], bump)]
    pub pda: Account<'info,Deposit>,
    pub system_program: Program<'info,System>
}

// #[derive(Accounts)]
// pub struct InitializeCounter<'info> {
//     #[account(init, space=8+8, payer=payer)]
//     pub counter: Account<'info, Counter>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }
