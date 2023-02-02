//Here we are getting all the different types we require from solana_program 

/*
*AcountInfo
*This is a struct 
*has fields that we might want to use like owner,is_signer
*has a data slice that is custom to the accout of type Rc<RefCell<&'a mut [u8]>>
*/

/*
*entrypoint 
*This is a macro which is the link between the program and the loder which is *supposed to run it 
*/

/*
*entrypoint::ProgramResult,z
*ProgramResult is a result type that can either be an empty turple or a *ProgramError enum type
*/

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};
use crate::processor::Processor ;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    //We take references as input data &Pubkey &[AccountInfo] &[u8] because we do
    // not want to consume the data 
    //forward the inputs for procssing 
    Processor::process(program_id, accounts, instruction_data)
}