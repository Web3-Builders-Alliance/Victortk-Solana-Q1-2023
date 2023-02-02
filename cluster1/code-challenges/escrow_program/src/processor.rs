use solana_program:: {
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg, // a macro used for logging to the blockchain for debugging 
    pubkey::Pubkey, //A turple struct type which is used to represent a publick key
    program_pack::{Pack, IsInitialized}, // Trait types used to make deserializing and serializing work 

    sysvar::{rent::Rent, Sysvar, clock::Clock}, //types used to access system parameters 

    program::{invoke, invoke_signed}, // these are functions you can use to make CPI's invoke extends signer privillages, invoke_signed is used to sign for a program 

    clock::Slot , //This is a type used to represent a number of ticks on the blockchain internally it is ust a u64  
};

use spl_token::state::Account as TokenAccount;

use crate::{error::EscrowError, instruction::EscrowInstruction, state::Escrow};
pub struct Processor;

impl Processor {
  pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8])-> ProgramResult {

      //The instruction data is serialized up to this point 
      //the instruction data is custom to a program 
      let instruction = EscrowInstruction::unpack(instruction_data)?;

      match instruction {
        EscrowInstruction::InitEscrow {amount} => {
          msg!("Instruction: InitEscrow");
          Self::process_init_escrow(accounts, amount, program_id)
        },
        EscrowInstruction::Exchange { amount } => {
          msg!("Instruction: Exchange");
          Self::process_exchange(accounts, amount, program_id)
        }
      }
  }

  fn process_init_escrow(
    accounts: &[AccountInfo],
    amount: u64,
    program_id: &Pubkey,

  ) -> ProgramResult {

    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;

    if !initializer.is_signer {
      return Err(ProgramError::MissingRequiredSignature);
    }

    let temp_token_account = next_account_info(account_info_iter)?;

    let token_to_receive_account = next_account_info(account_info_iter)?;

    if *token_to_receive_account.owner != spl_token::id() {
      return Err(ProgramError::IncorrectProgramId);
    }

    let escrow_account = next_account_info(account_info_iter)?;
    let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
  
    if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
      return Err(EscrowError::NotRentExempt.into());
    }

    let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.try_borrow_data()?)?;

    if escrow_info.is_initialized() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let clock:Slot = Clock::get()?.slot ;
    let unlock_time:Slot = clock + 100 ;
    let time_out:Slot = unlock_time + 1000;

    escrow_info.is_initialized = true ;
    escrow_info.initializer_pubkey = *initializer.key ;
    escrow_info.temp_token_account_pubkey = *temp_token_account.key;
    escrow_info.initializer_token_to_receive_account_pubkey = *token_to_receive_account.key ;
    escrow_info.expected_amount = amount ;
    escrow_info.unlock_time = unlock_time ;
    escrow_info.time_out = time_out ;


    Escrow::pack(escrow_info, &mut escrow_account.try_borrow_mut_data()?)?;
    let (pda, _bump_seed) = Pubkey::find_program_address(&[b"escrow"], program_id);
    
    let token_program = next_account_info(account_info_iter)?;
    let owner_change_ix = spl_token::instruction::set_authority(
      token_program.key,
      temp_token_account.key,
      Some(&pda),
      spl_token::instruction::AuthorityType::AccountOwner,
      initializer.key,
      &[&initializer.key],
    )?;

    msg!("Calling the token program to transfer token account ownership...");

    invoke(
      &owner_change_ix,
      &[
        temp_token_account.clone(),
        initializer.clone(),
        token_program.clone(),
      ],
    )?;

    Ok(())
  }

  fn process_exchange(
    accounts: &[AccountInfo],
    amount_expected_by_taker: u64,
    program_id: &Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let taker = next_account_info(account_info_iter)?;

    if !taker.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let takers_sending_token_account = next_account_info(account_info_iter)?;

    let takers_token_to_receive_account = next_account_info(account_info_iter)?;

    let pdas_temp_token_account = next_account_info(account_info_iter)?;

    let pdas_temp_token_account_info = TokenAccount::unpack(&pdas_temp_token_account.try_borrow_data()?)?;

    let (pda, bump_seed) = Pubkey::find_program_address(&[b"escrow"], program_id);

    if amount_expected_by_taker != pdas_temp_token_account_info.amount {
        return Err(EscrowError::ExpectedAmountMismatch.into());
    }

    let initializers_main_account = next_account_info(account_info_iter)?;
    let initializers_token_to_receive_account = next_account_info(account_info_iter)?;
    let escrow_account = next_account_info(account_info_iter)?;

    let escrow_info = Escrow::unpack(&escrow_account.try_borrow_data()?)?;

    if escrow_info.temp_token_account_pubkey != *pdas_temp_token_account.key {
        return Err(ProgramError::InvalidAccountData);
    }

    if escrow_info.initializer_pubkey != *initializers_main_account.key {
        return Err(ProgramError::InvalidAccountData);
    }

    if escrow_info.initializer_token_to_receive_account_pubkey != *initializers_token_to_receive_account.key {
        return Err(ProgramError::InvalidAccountData);
    }

    let clock = Clock::get()?;
        
    if escrow_info.unlock_time > clock.slot {
         return Err(EscrowError::EscrowNotUnlocked.into());
    }

    if escrow_info.time_out < clock.slot {
         return Err(EscrowError::EscrowTimedOut.into());
    }

    let token_program = next_account_info(account_info_iter)?;

    let transfer_to_initializer_ix = spl_token::instruction::transfer(
        token_program.key,
        takers_sending_token_account.key,
        initializers_token_to_receive_account.key,
        taker.key,
        &[&taker.key],
        escrow_info.expected_amount,
    )?;
    msg!("Calling the token program to transfer tokens to the escrow's initializer...");
    invoke(
        &transfer_to_initializer_ix,
        &[
            takers_sending_token_account.clone(),
            initializers_token_to_receive_account.clone(),
            taker.clone(),
            token_program.clone(),
        ],
    )?;
    let pda_account = next_account_info(account_info_iter)?;

    let transfer_to_taker_ix = spl_token::instruction::transfer(
    token_program.key,
    pdas_temp_token_account.key,
    takers_token_to_receive_account.key,
    &pda,
    &[&pda],
    pdas_temp_token_account_info.amount,
    )?;

    msg!("Calling the token program to transfer tokens to the taker...");

    invoke_signed(
    &transfer_to_taker_ix,
    &[
        pdas_temp_token_account.clone(),
        takers_token_to_receive_account.clone(),
        pda_account.clone(),
        token_program.clone(),
    ],
    &[&[&b"escrow"[..], &[bump_seed]]],
   )?;

  let close_pdas_temp_acc_ix = spl_token::instruction::close_account(
    token_program.key,
    pdas_temp_token_account.key,
    initializers_main_account.key,
    &pda,
    &[&pda]
  )?;
  msg!("Calling the token program to close pda's temp account...");

  invoke_signed(
    &close_pdas_temp_acc_ix,
    &[
        pdas_temp_token_account.clone(),
        initializers_main_account.clone(),
        pda_account.clone(),
        token_program.clone(),
    ],
    &[&[&b"escrow"[..], &[bump_seed]]],
  )?;

  Ok(())
  }
}