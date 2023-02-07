// inside instruction.rs 
use std::convert::TryInto ;
use solana_program::{program_error::ProgramError, pubkey::Pubkey , instruction::{Instruction, AccountMeta}, sysvar::rent::Rent} ;
use crate::error::EscrowError::InvalidInstruction ;
use std::mem::size_of ;

// 0. `[signer]` The account of the person initializing the escrow
  /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer 
  /// 2. `[]` The initializer's token account for the token they will recieve should the trade go through
  /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade 
  /// 4. `[]` The rent sysvar 
  /// 5. `[]` The token program

pub fn init_escrow(
  program_id: &Pubkey,
  initiator: &Pubkey,
  temp_token_account: &Pubkey,
  init_token_to_receive_account: &Pubkey,
  escrow_account: &Pubkey,
  token_program: &Pubkey,
  amount: u64,
)-> Result<Instruction, ProgramError> {

  let init_data = EscrowInstruction::InitEscrow{
    amount,
  };
  let accounts = vec![
    AccountMeta::new(*initiator, true),
    AccountMeta::new(*temp_token_account, false),
    AccountMeta::new(*init_token_to_receive_account,false),
    AccountMeta::new(*escrow_account,false),
    AccountMeta::new_readonly(*token_program, false),
  ];

  Ok(Instruction {
    program_id: *program_id,
    accounts,
    data: init_data.pack()
  })
}

pub fn exchange (
  program_id: &Pubkey,
  taker: &Pubkey,
  taker_token_account: &Pubkey ,
  taker_token_account2 : &Pubkey ,
  temp_token_account: &Pubkey,
  initializer_token_account: &Pubkey,
  initializer_main_account: &Pubkey,
  escrow_account: &Pubkey,
  token_program: &Pubkey,
  amount: u64
) -> Result<Instruction, ProgramError> {

  let data = EscrowInstruction::Exchange {
    amount, 
  }.pack() ;

  //#### How to decide if account is read_only or nah?
  let accounts = vec![
    AccountMeta::new(*taker, true),
    AccountMeta::new(*taker_token_account, false),
    AccountMeta::new(*taker_token_account2, false),
    AccountMeta::new(*temp_token_account, false),
    AccountMeta::new(*initializer_token_account, false),
    AccountMeta::new(*initializer_main_account, false),
    AccountMeta::new(*escrow_account, false),
    AccountMeta::new_readonly(*token_program, false) ,
  ];

  Ok(
    Instruction {
      program_id: *program_id,
      accounts,
      data
    }
  )
} 
pub enum EscrowInstruction {

  /// Starts the trade by creating and populating an escrow account and transferring ownership of the given temp token account to the PDA
  /// 
  /// 
  /// Accounts expected:
  /// 
  /// 0. `[signer]` The account of the person initializing the escrow
  /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer 
  /// 2. `[]` The initializer's token account for the token they will recieve should the trade go through
  /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade 
  /// 4. `[]` The rent sysvar 
  /// 5. `[]` The token program 
  
  InitEscrow {
    ///The amount party A expects to receive of token Y 
    amount: u64  
  },

  /// Accepts a trade
  ///
  ///
  /// Accounts expected:
  ///
  /// 0. `[signer]` The account of the person taking the trade
  /// 1. `[writable]` The taker's token account for the token they send 
  /// 2. `[writable]` The taker's token account for the token they will receive should the trade go through
  /// 3. `[writable]` The PDA's temp token account to get tokens from and eventually close
  /// 4. `[writable]` The initializer's main account to send their rent fees to
  /// 5. `[writable]` The initializer's token account that will receive tokens
  /// 6. `[writable]` The escrow account holding the escrow info
  /// 7. `[]` The token program
  /// 8. `[]` The PDA account
  Exchange {
    /// the amount the taker expects to be paid in the other token, as a u64 because that's the max possible supply of a token
    amount: u64,
  },

  // signer 
  ///Cancel Escrow 
  /// 0. `[signer] The initializer that is cancelling their escrow 
  /// 1. `[writable]` The program's token acount
  /// 2. `[writable]` The initializer's token avount that will receive tokens 
  /// 3. `[writable]` The escrow account holding the escro info 
  /// 4. `[]` The token program
  /// 5. `[]` The PDA account
  Cancel {},
  
  //Reset Time lock and time_out 
  ////0. `[signer]` The initializer that is reseting the timelock 
  /// 1. `[writable]` The escrow account holding the escrow info 
  ResetTimeCount {}

  //
}

impl EscrowInstruction {

  /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
  pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        
        //We know how the data we expect is structured and thats how we know
        //the right way to deserialize it 
       //Split first takes the first byte out of the slice and
       // returns the rest of the slice and the first byte in a tuple

      let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

      // here we use a match to make a descision on how to deserialize the 
      //rest of the byte array
      Ok(match tag {
        //if the tag is zero we run code to innitialize
        0 => Self::InitEscrow{ 
          amount: Self::unpack_amount(rest)?
        },
        //if the tag is 1 we run the code to exchange 
        1 => Self::Exchange {
           amount: Self::unpack_amount(rest)?
        },
        _ => return Err(InvalidInstruction.into()),   
      })
  }

  fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
     //this fuction takes a byte array 
     //uses the first 8 bytes to create a u64 value
     //Returns the amount 
     
     let amount = input 
         .get(..8)
         .and_then(|slice| slice.try_into().ok())
         .map(u64::from_le_bytes)
         .ok_or(InvalidInstruction)?;
         Ok(amount)
  }

  pub fn pack(&self) -> Vec<u8> {
    let mut buf = Vec::with_capacity(size_of::<Self>());
    match &*self {
      Self::InitEscrow {amount} => {
        buf.push(0);
        buf.extend_from_slice(&amount.to_le_bytes());
      },
      Self::Exchange {amount} => {
        buf.push(1);
        buf.extend_from_slice(&amount.to_le_bytes());
      },
      Self::ResetTimeCount {} => {
        buf.push(2);
      },
      Self::Cancel {} => {
        buf.push(3);
      }
    }
    buf
  }


}