'use client'
import React from 'react';
import * as anchor from '@project-serum/anchor';
import { Farmer, IDL } from '../programs/farmer'; 
import { Connection, Keypair, SystemProgram,PublicKey } from '@solana/web3.js';
import {Box, Button} from "@mui/material";
import styles from './page.module.css';
import {
	AnchorWallet,
	useWallet,
	useAnchorWallet,
} from '@solana/wallet-adapter-react';
import {
	Program,
	Wallet,
	AnchorProvider,
	Address,
	BN,
} from '@project-serum/anchor';
const CreateFarmer = () => {
	const { select, wallets, publicKey,wallet, disconnect, signTransaction, signAllTransactions } = useWallet();
  // const wallet =   useWallet()https://api.devnet.solana.comhttp://localhost:8899
	const w = useAnchorWallet();
	const connection = new Connection('https://api.devnet.solana.com');  
	const provider = new AnchorProvider(
		connection,
		w as Wallet,
		{
			commitment: 'confirmed',
		}
	);
	const programID = new PublicKey("5LJq1WKXV2bdgsosp6wk2pgvk1Rhc75ffRLRXGZvPQWU");
  const program = new Program(IDL, programID, provider); 
	const handleCreate = async () => {
		console.log("click clack");
			let name = 'Jonh Doee';
			try{
				if(publicKey) {
					let [farmer] = anchor.web3.PublicKey.findProgramAddressSync(
					[Buffer.from('farmer'), publicKey.toBuffer()],
					program.programId
				);
				// const tx = await program.methods
				// 	.initializeFarmer(name)
				// 	.accounts({
				// 		farmer,
				// 	})
				// 	.rpc();
				// console.log('Your transaction signature', tx);
   			
				let farmerState = await program.account.farmer.fetchNullable(farmer);				
				if (farmerState){
				console.log('the famer name is,', farmerState.name);
				console.log('the famer address is,', farmerState.address.toString());
				console.log(
					'the famer land count is,',
					farmerState.landCount.toString()
				);
				console.log(
					'the famer tree count is,',
					farmerState.treeCount.toString()
				);
				}else {
					console.log("the state is", farmerState)
				}			
				}else{
					throw("Connect to wallet")
				}        
			}catch(e){
       console.log('There was an error ', e);
			}					
	}
	return(
		<Box className={styles.box}>
			<legend>Create A Farmer Account</legend>
			<label htmlFor="name"></label><input className={styles.name}  type='text' name='name' id ="name"/>
			<Button onClick={handleCreate}>Register Famer</Button>
		</Box>
	);
}

export default CreateFarmer