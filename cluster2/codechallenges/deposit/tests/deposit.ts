import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import * as web3 from "@solana/web3.js";
import {ASSOCIATED_PROGRAM_ID,TOKEN_PROGRAM_ID} from '@project-serum/anchor/dist/cjs/utils/token';
import { Deposit } from "../target/types/deposit";
import { expect } from 'chai';
import * as token from "@solana/spl-token" ;
import { BN } from "bn.js";
import {Metaplex} from "@metaplex-foundation/js";


describe("deposit", () => {
  // Configure the client to use the local cluster.
  const anchorProvider = anchor.AnchorProvider.env() ;
  anchor.setProvider(anchorProvider);

  const program = anchor.workspace.Deposit as Program<Deposit>;
  const metaplex = Metaplex.make(anchorProvider.connection) ;

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
  let amount = new anchor.BN(web3.LAMPORTS_PER_SOL) ;

  it("Is Initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
			.initialize(new anchor.BN(web3.LAMPORTS_PER_SOL))
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
          .deposit(new anchor.BN(web3.LAMPORTS_PER_SOL * 1.5))
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
          .withdraw(new anchor.BN(web3.LAMPORTS_PER_SOL * .75))
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
     

     it("deposits token", async() => {

       //create a mint 
      const tokenMint = await token.createMint(
      anchorProvider.connection,
      (anchorProvider.wallet as anchor.Wallet).payer,
      anchorProvider.publicKey,
      anchorProvider.publicKey,
      6      
      );

      let tkMint = await token.getMint(anchorProvider.connection, tokenMint);
       console.log("mint: ", tkMint);
      // const  user_token_account = web3.Keypair.generate();

      const [pdaAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
       [Buffer.from("tokenvault"),
       anchorProvider.publicKey.toBuffer(),
       tokenMint.toBuffer()],
       program.programId
     )
     const pdaAssociatedTokenAcc =
				await token.getOrCreateAssociatedTokenAccount(
					anchorProvider.connection,
					(anchorProvider.wallet as anchor.Wallet).payer,
					tokenMint,
					pdaAuthority,
					true
				);

     const depositAssociatedTokenAcc =
				await token.getOrCreateAssociatedTokenAccount(
					anchorProvider.connection,
					(anchorProvider.wallet as anchor.Wallet).payer,
					tokenMint,
					anchorProvider.publicKey
				);

   /*     
     const depositAssociatedTokenAcc =
				await token.getAssociatedTokenAddress(
					tokenMint,
					anchorProvider.publicKey
          );
   */		



      await token.mintTo(
				anchorProvider.connection,
				(anchorProvider.wallet as anchor.Wallet).payer,
				tokenMint,
				depositAssociatedTokenAcc.address,
				anchorProvider.publicKey,
				30000000
			);

      const tx = await program.methods.depositTokens(new anchor.BN(2e6)).accounts({
				tokenMint,
				depositAuthority:anchorProvider.publicKey,
				pdaAssociatedTokenAcc: pdaAssociatedTokenAcc.address,
				pdaAuthority,
				depositAssociatedTokenAcc: depositAssociatedTokenAcc.address,
				associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
				tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
			}).rpc();

      console.log("Your Transaction is :", tx);

     let acc  = await  token.getAccount(anchorProvider.connection, pdaAssociatedTokenAcc.address);     

     console.log("the pda balance is now:", acc.amount.toString());
     
  })

  it("Deposits Nft", async () => {
		//create a nft mint
		const nftMint = await token.createMint(
			anchorProvider.connection,
			(anchorProvider.wallet as anchor.Wallet).payer,
			anchorProvider.publicKey,
			anchorProvider.publicKey,
			0
		);

     const [pdaAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
				[
					Buffer.from('tokenvault'),
					anchorProvider.publicKey.toBuffer(),
					nftMint.toBuffer(),
				],
				program.programId
			);
			const pdaAssociatedNftAcc =
				await token.getOrCreateAssociatedTokenAccount(
					anchorProvider.connection,
					(anchorProvider.wallet as anchor.Wallet).payer,
					nftMint,
					pdaAuthority,
					true
				);

			const depositAssociatedNftAcc =
				await token.getOrCreateAssociatedTokenAccount(
					anchorProvider.connection,
					(anchorProvider.wallet as anchor.Wallet).payer,
					nftMint,
					anchorProvider.publicKey
				);


        metaplex.nfts().create({
          name:"CoolSks",
          symbol: "Sks",
          uri:"arweav.uri",
          creators:[{}]
        })

       

	})




  
});
