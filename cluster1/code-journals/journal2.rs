
//#### we use the BorshDeserialize to convert from bytes to types that we can use in our program 
use borsh::BorshDeserialize;
//#### We a getting a Type SetPowerStatus from lever 
use lever::SetPowerStatus;
//#### Here we are getting types that we need from solana_program library
//#### snake_case convention for functions like next_account_info
//#### Camel case for custom types like AccountInfo which is a struct 
use solana_program::{
    account_info::{ 
        next_account_info, AccountInfo
    },
    entrypoint, //#### macros expand into more code at compile time 
    entrypoint::ProgramResult, 
    instruction::{ AccountMeta, Instruction },//#### AccountMeta to mark accounts as writtable or
                                            //####as signers
    pubkey::Pubkey,
    program::invoke, //#### invoke is a function used to call a program from another program 
};


entrypoint!(pull_lever);//#### the entrypoint to this function is called pull_lever
//#### you can name the entrypoint function anything you want 


fn pull_lever(
    _program_id: &Pubkey, //#### we are not using the program id in the function and  
                          //#### that is why there there is an underscore preceeding its name  
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    
    //#### here we create an iterator over the accounts 
    let accounts_iter = &mut accounts.iter();

    //#### we should check if the power account is initialized 
    let power = next_account_info(accounts_iter)?; //#### we already know the order that the  
                                                //####accounts will be passed to the program. 
    let lever_program = next_account_info(accounts_iter)?;//#### the ? is used to propagate the error
                                                          //#### unwrap the result  

    let set_power_status_instruction = SetPowerStatus::try_from_slice(instruction_data)?;

    let ix = Instruction::new_with_borsh(
        lever_program.key.clone(),                          // Our lever program's ID
        &set_power_status_instruction,                      // Passing instructions through
        vec![AccountMeta::new(power.key.clone(), false)],   // Just the required account for the other program
    );

    invoke(&ix, &[power.clone()]) //#### invoke is used to call another program from this program 
                                  //#### the program to be called should be included in the accounts 
                                  //#### slice 
                                  //####All account previlleges are propagated to the program being called 
                                  //#### An account marked as signer or writable will be a signer writable 
                                  //####in the program called aswell
                                  //####invoke is used when the program is not the signer in the account  
}