use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
    account_info::{
        //#### iterate over AccountInfo safely with next_account_info
        next_account_info, AccountInfo
    },
    entrypoint, 
    entrypoint::ProgramResult, //#### type ProgramResult = Result<(),ProgramError>
    msg, 
    program::invoke,//#### invoke is used to make a CPI call, it extends previllages it just 
                    //#### passes on all the AccountMeta  
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,//#### this module we can use for testing and writing Instruction builders   

    sysvar::Sysvar, //####This is where we can get system variables  modules like clock , rent ,  
};

//#### entrypoint could be moved from this file and into an entrypoint.rs file 
#[cfg(not(feature = "no-entrypoint"))] //####this configuration attribute used 
entrypoint!(process_instruction);      //####so that we entrypoint is not included when the 
                                       //####the program is called from a cpi

//#### the process_instruction function could be put in a file called process_instruction 
pub fn process_instruction(
    program_id: &Pubkey, //#### struct Pubkey(_)
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult { //####type ProgramResult = Result<(), ProgramError> 

    //#### try_from_slice is a function provided by borsh to deserialize data.
    //#### In this case to deserialize PowerStatus
    match PowerStatus::try_from_slice(&instruction_data) { //####try_from_slice returns a result 
        //####which we match in the arms of the macth statement 
        Ok(power_status) => return initialize(program_id, accounts, power_status),
        Err(_) => {},
    }

    //#### This section of the code is reached if we did not successfully deserialize the Powerstatus
    //####try_from_slice takes a slice so that it does not consume the data 
    // 
    match SetPowerStatus::try_from_slice(&instruction_data) {
        Ok(set_power_status) => return switch_power(accounts, set_power_status.name),
        Err(_) => {},
    }

    Err(ProgramError::InvalidInstructionData) //#### if the program reaches here then 
                                              //#### Both instructions failed to deserialize 
    
    //#### A better way is to use the first byte of the instruction data as a tag
    //#### If the tag is Zero deserialize PowerStatus 
    //#### if the tag is One  deserialize SetPowerStatus
    //#### then call the appropriate processing functions 
}



//## in this function we are creating a system account  
//#### we then use the account to store PowerStatus in a serilized way 
//#### in the data slice of the account 
pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    power_status: PowerStatus,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();

    //#### we should check if this account is not already initialized 
    let power = next_account_info(accounts_iter)?;

    //#### we should check if the user is signing the transaction 
    let user = next_account_info(accounts_iter)?;

    //## we should check if this is really the system program 
    let system_program = next_account_info(accounts_iter)?;

    //#### here we calculate the rent required to make the Power Account 
    //#### rent exempt 
    let account_span = (power_status.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    //#### Invoke is used to make a CPI to the system_program 
    //#### signer previllages are extended to the system_program  
    invoke(
        &system_instruction::create_account(
            &user.key,
            &power.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[
            user.clone(), power.clone(), system_program.clone()
        ]
    )?;

    //#### here we borrow_mutably a slice to the data field of the account we just created
    //##### we then serialize the current power status onn it 
    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?;

    Ok(())
}

//#### function takes in the name of the person who is pulling the switch
//#### if the swicth is off it is switched on if it is off it is switched on 
pub fn switch_power(
    accounts: &[AccountInfo],
    name: String,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let power = next_account_info(accounts_iter)?;//#### we should check if this is the actuall
    //#### account we want before using it 

    let mut power_status = PowerStatus::try_from_slice(&power.data.borrow())?;
    

    power_status.is_on = !power_status.is_on; //#### true -> false false -> true with not ! operator

    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?; //#### borrowing a slice 
    //#### to the data, the entire slice [..] mutating it with the serialized version of 
    //#### power_status

    msg!("{} is pulling the power switch!", &name); //### logging to the blockchain 

    //#### match used as a conditional
    //#### to print different things for true or false
    match power_status.is_on {
        true => msg!("The power is now on."),//#### msg macro used to logg on the blockchain 
        false => msg!("The power is now off!"),
    };

    Ok(()) //#### return the result Ok(()) for successful exit 
}


//#### attributes expand into code that is required by the borsh trait
//#### attribtes like macros are a form of metaprograming 
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct SetPowerStatus {
    pub name: String, //#### borsh knows how to deserialize Strings so there is no need for it us 
    //#### to make custom functions 
}

//#### PowerStatus marked wwith attributes so that it can be deserialized and serialized by borsh  
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PowerStatus {
    pub is_on: bool, //#### bool can either be true or false
}

//#### structs in rust are the basic way of grouping data
//#### PowerStatus and SetPowerStatus are traditional structs 
//#### fields, variants in the struct are marked as pub so that they can be accesssed from outside
//#### the crate 
