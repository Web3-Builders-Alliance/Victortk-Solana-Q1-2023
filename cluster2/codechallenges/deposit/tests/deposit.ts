import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { Deposit } from "../target/types/deposit";
import { expect } from 'chai';

describe("deposit", () => {
  // Configure the client to use the local cluster.
  const anchorProvider = anchor.AnchorProvider.env() ;
  anchor.setProvider(anchorProvider);

  const program = anchor.workspace.Deposit as Program<Deposit>;

  const seed = 'vault';
   
  //we want a user key provider 
  const payer = anchorProvider.wallet ;
  // we want to create a pda that we can send sol to 
  const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from(seed),payer.publicKey.toBuffer()],
		program.programId
	);
    console.log(pda.toString())
  // we need an amount to send  
  let amount = new anchor.BN(LAMPORTS_PER_SOL) ;

  it("Is Initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
			.initialize(new anchor.BN(LAMPORTS_PER_SOL))
			.accounts({
				authority: payer.publicKey,
				vault:pda,
				systemProgram: anchor.web3.SystemProgram.programId,
			})
			.rpc();

    console.log("Your transaction signature", tx);
     let balance =  await anchorProvider.connection.getBalance(pda) ;
     console.log("initialized with balance :", balance)                

    });
    it("Deposits", async() => {

        const tx2 = await program.methods
          .deposit(new anchor.BN(LAMPORTS_PER_SOL * 1.5))
          .accounts({
            authority: payer.publicKey,
            vault: pda,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();
          console.log('Your transaction signature', tx2);
          let balance = await anchorProvider.connection.getBalance(pda);
					console.log('Deposit some,new balance :', balance);  
       

        });
    it("Withdraws", async() => {
        const tx3 = await program.methods
          .withdraw(new anchor.BN(LAMPORTS_PER_SOL * .75))
          .accounts({
            authority: payer.publicKey,
            vault: pda,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();
          console.log('Your transaction signature', tx3);
          let balance = await anchorProvider.connection.getBalance(pda);
					console.log('withdraw some, new balance :', balance);  
          

        });

    it("Closes", async() => {
        const tx4 = await program.methods
          .close()
          .accounts({
            authority: payer.publicKey,
            vault: pda,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();
          console.log('Your transaction signature', tx4)
          let balance = await anchorProvider.connection.getBalance(pda);
					console.log('closed balance :', balance);  

        });
  
});
