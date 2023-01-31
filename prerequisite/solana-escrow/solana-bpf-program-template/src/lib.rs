pub mod instruction ;
pub mod error ;
pub mod processor ;
pub mod state ;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

//program flow 
// Someone calls the entrypoint 
// The entrypoint forwards the arguments to the processor
//The processor asks instruction.rs to decode the instruction_data argument from the entrypoint function.
//Using the decoded data, the processor will now decide which processing function to use to process the request.
// the processor may use state.rs to encode state into or decode the state of an account which has been passed into the entrypoint.


