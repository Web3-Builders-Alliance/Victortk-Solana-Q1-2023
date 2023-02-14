
#[derive(Accounts)]
pub struct DepositToken<'info> {    
 #[account(mut)] 
 pub token_mint: Account<'info, Mint> ,
 #[account(mut)] 
 pub deposit_authority: Signer<'info> ,
 #[account( init_if_needed, associated_token::mint = token_mint, payer = deposit_authority,associated_token::authority = pda_authority,
 )] 
 pub pda_associated_token_acc: Account<'info, TokenAccount>,
 #[account(    
    seeds=[b"tokenvault",deposit_authority.to_account_info().key.as_ref(),token_mint.to_account_info().key.as_ref()],bump)]   
 pub pda_authority: UncheckedAccount<'info>,
 #[account(     
    associated_token::mint = token_mint,   
    associated_token::authority = deposit_authority,
 )]
 pub deposit_associated_token_acc: Account<'info, TokenAccount>, 
 pub associated_token_program: Program<'info, AssociatedToken>, 
 pub token_program: Program<'info,Token>, 
 pub system_program: Program<'info,System> 
}