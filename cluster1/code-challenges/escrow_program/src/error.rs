use thiserror::Error ;
use solana_program::program_error::ProgramError;

#[derive(Error,Debug, Copy, Clone)]
pub enum EscrowError {
  ///Invalid instruction
 #[error("Invalid Instruction")]
 InvalidInstruction,
 
 //Not rent Exempt 
 #[error("Not rent Exempt")]
 NotRentExempt,

 //Expected Amount Mismatch
 #[error("Expected Amount Mismatch")]
 ExpectedAmountMismatch,

 //Escrow Not Unlocked Yet
 #[error("Escrow Not Unlocked Yet")]
 EscrowNotUnlocked,

 //Escrow Timed Out
 #[error("Escrow Timed Out")]
 EscrowTimedOut,

}

impl From<EscrowError> for ProgramError {
  fn from(e: EscrowError) -> Self {
    ProgramError::Custom(e as u32)
  }

}