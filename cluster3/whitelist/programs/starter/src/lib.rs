use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod starter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.whitelist.authority = ctx.accounts.payer.to_account_info().key() ;
        ctx.accounts.whitelist.bumps = *ctx.bumps.get("whitelist").unwrap();
        Ok(())
    }

    pub fn add_to_whitelist (ctx: Context<AddToWhitelist>) -> Result<()> {
       Ok(ctx.accounts.whitelist_pda.bumps = *ctx.bumps.get("whitelist_pda").unwrap())       
    }

    pub fn remove_to_whitelist (_ctx: Context<RemoveFromWhitelist>) -> Result<()> {
        Ok(())
    }
}


#[account]
pub struct Whitelist {
    pub authority : Pubkey, 
    pub bumps :u8
}

#[derive(Accounts)]
pub struct Initialize <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer=payer, space = 8 + 8 + 32, seeds=[b"whitelist" , payer.key().as_ref()], bump )]
    pub whitelist: Account<'info,Whitelist>,
    pub system_program: Program<'info,System>,
}

#[account]
pub struct WhitelistPDA {
    pub bumps: u8
}

#[derive(Accounts)]
pub struct AddToWhitelist <'info>{
   #[account(mut)]
    pub authority: Signer<'info>, 
    pub to_be_whitelisted: SystemAccount<'info>,
    #[account(seeds=[b"whitelist", authority.key().as_ref()], bump=whitelist.bumps, has_one=authority)]
    pub whitelist: Account<'info,Whitelist>,
    #[account(init, payer=authority, space = 8 + 8, seeds=[whitelist.key().as_ref(),to_be_whitelisted.key().as_ref() ], bump )]
    pub whitelist_pda: Account<'info,WhitelistPDA>,
    pub system_program: Program<'info,System>,
}

#[derive(Accounts)]
pub struct RemoveFromWhitelist <'info>{
   #[account(mut)]
    pub authority: Signer<'info>, 
    pub to_be_removed: SystemAccount<'info>,
    #[account(seeds=[b"whitelist", authority.key().as_ref()], bump=whitelist.bumps, has_one=authority)]
    pub whitelist: Account<'info,Whitelist>,
    #[account(mut, close=authority, seeds=[whitelist.key().as_ref(),to_be_removed.key().as_ref() ], bump )]
    pub whitelist_pda: Account<'info,WhitelistPDA>,
    pub system_program: Program<'info,System>,
}
