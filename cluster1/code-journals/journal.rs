//####the keyword use is used to shorten the path when calling types from another module or crate 
//####You can use a different diffferent name for the type or item when you bring it in
//####scope with the as keyword
use solana_program::{
    //####account_info is a module in solana program library 
    //####AccountInfo is a struct that is the represantation of the structure of account data on chain
    //####next_account_info is a function used to iterate over accounts 
    account_info::{ AccountInfo, next_account_info }, 
    entrypoint, //####entrypoint is a macro it expands into more code which is like the
                //####main for the program, it matches the program to the right loader 
    entrypoint::ProgramResult,  //####ProgramResult is a Result type which can either be an empty tuple
                                //#### or a ProgramError type
    msg, //#### msg is a macro used for loggong to on the blockchain useful for debugging 
    program_error::ProgramError, //####This is an enum type that lists different errors that the program
                                 //####can return you can custom variants to the list 
    pubkey::Pubkey,//#### Pubkey is a struct type used to represent a publick Key on chain 
    system_program,//####native solana program used which owns all system accounts which
                   //#### are not executable
};

//####we could make this better by using an attribute to exclude the entrypoint 
//####when calling from a cpi #[cfg(not(feature="no-entrypoint"))]
entrypoint!(process_instruction); //####macro expands into more code , meta programing 


fn process_instruction(
    program_id: &Pubkey, //#### program_id is a reference of a pubkey so that it does not
                         //#### take ownwership of the Pubkey thereby destroying it when function
                         //#### scope ends
    accounts: &[AccountInfo],//####accounts slice
    _instruction_data: &[u8],//#### slice of data that only the program knows how to deserialize
                            //#### in this case we have the _underscore because we do not use
                            //#### instruction data in the function 
) -> ProgramResult { //#### The return type of this  function is ProgramResult

    //You can verify the program ID from the instruction is in fact 
    //the program ID of your program.
    if system_program::check_id(program_id) { //##### is it possible to pass the wrong program id and still be  
        //#### able to call A particular program?
        //#### I later thought that maybe it is possible if we call the function via cpi??
        return Err(ProgramError::IncorrectProgramId) //####We can eturn an Err using Err
    };
    
    // You can verify the list has the correct number of accounts.
    //This error will get thrown by default if you 
    // try to reach past the end of the iter.
    if accounts.len() < 4 {
        msg!("This instruction requires 4 accounts:");
        msg!("  payer, account_to_create, account_to_change, system_program");
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    // Accounts passed in a vector must be in the expected order.
    let accounts_iter = &mut accounts.iter(); //##### taking a mutable reference to the 
    //#### accounts so that we can mutate the values we get from the iterator. 
    //#### Iterating over vectors is safer than indexing

    let _payer = next_account_info(accounts_iter)?; //#### the first account is the payer, 
    //#### it is not being used in the program that is why there is an underscore on the 
    //#### variable name 

    //#### we can make this better by using the payer and checking if it signed the 
    //#### transaction payer.is_signer should be true

    
    //#### next account info returns the next account in the iterator 
    //#### we already know what the order of the accounts being passed is before hand
    let account_to_create = next_account_info(accounts_iter)?;
    let account_to_change = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // You can make sure an account has NOT been initialized.

    //#### we can also use an is_initialized field on account_to_create
    //#### We could make here better by including a is_initialized field on the account_to_create
    msg!("New account: {}", account_to_create.key); //#### msg macro used to log to the blockchain 
    if account_to_create.lamports() != 0 { 
        msg!("The program expected the account to create to not yet be initialized.");
        return Err(ProgramError::AccountAlreadyInitialized) 
    };
    // (Create account...)

    // You can also make sure an account has been initialized.
    msg!("Account to change: {}", account_to_change.key);
    if account_to_change.lamports() == 0 { //#### test for equality with double equal signs
        msg!("The program expected the account to change to be initialized.");
        return Err(ProgramError::UninitializedAccount) //####return an error if condition is met
                                                      //#### or true
    };

    // If we want to modify an account's data, it must be owned by our program.
    if account_to_change.owner != program_id {
        msg!("Account to change does not have the correct program id.");
        return Err(ProgramError::IncorrectProgramId)
    };

    // You can also check pubkeys against constants.
    if system_program.key != &system_program::ID { //#### not equal comparison operator !=
        return Err(ProgramError::IncorrectProgramId)
    };

    //####this code does not do anything it just returns if the accounts passed in are correct
    //####The last statement in a file if it does not have a semi colon it is returned automatically
    //#### which is an Ok(()) with an empty tuple insite 
    //This is like how a script returns 0 for successful exit 
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
      
    }
}
