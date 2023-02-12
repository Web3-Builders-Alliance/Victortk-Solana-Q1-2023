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
  const [pda ]= anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(seed)],
    program.programId    
    )
    console.log(pda.toString())
  // we need an amount to send  
  let amount = new anchor.BN(LAMPORTS_PER_SOL) ;

  it("Is Initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
			.initialize(new anchor.BN(LAMPORTS_PER_SOL))
			.accounts({
				owner: payer.publicKey,
				vault:pda,
				systemProgram: anchor.web3.SystemProgram.programId,
			})
			.rpc();
    console.log("Your transaction signature", tx);
    console.log('The balance is now?', await (await program.account.vault.fetch(pda)).balance.toNumber()/LAMPORTS_PER_SOL);

     expect((await program.account.vault.fetch(pda)).isInitialized).to.equal(true);
 
    

    });
    it("Deposits", async() => {

        const tx2 = await program.methods
          .deposit(new anchor.BN(LAMPORTS_PER_SOL * .5))
          .accounts({
            owner: payer.publicKey,
            vault: pda,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();
          console.log('Your transaction signature', tx2);
          console.log(
            'The balance is now?',
            await(await program.account.vault.fetch(pda)).balance.toNumber()              
          );

        });
  
});
