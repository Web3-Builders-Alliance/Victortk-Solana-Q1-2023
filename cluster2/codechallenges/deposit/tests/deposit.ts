import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { Deposit } from "../target/types/deposit";

describe("deposit", () => {
  // Configure the client to use the local cluster.
  const anchorProvider = anchor.AnchorProvider.env() ;
  anchor.setProvider(anchorProvider);

  const program = anchor.workspace.Deposit as Program<Deposit>;

  const seed = "depositssss" ;
   
  //we want a user key provider 
  const payer = anchorProvider.wallet ;
  // we want to create a pda that we can send sol to 
  const [pda ]= anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(seed)],
    program.programId    
    )
    console.log(pda.toString())
  // we need an amount to send  
  let amount = new anchor.BN(LAMPORTS_PER_SOL) ;

  it("Is deposits!", async () => {
    // Add your test here.
    const tx = await program.methods
			.initialize(amount)
			.accounts({
				payer: payer.publicKey,
				pda,
				systemProgram: anchor.web3.SystemProgram.programId,
			})
			.rpc();
    console.log("Your transaction signature", tx);
  });
});
