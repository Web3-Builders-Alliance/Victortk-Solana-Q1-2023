'use client';
import React, {useState} from 'react';
import { Button, Box, Typography, TextField } from '@mui/material';
import styles from "./page.module.css";
import * as anchor from '@project-serum/anchor';
import { useAnchorWallet } from '@solana/wallet-adapter-react';
import { Program, Wallet, AnchorProvider } from '@project-serum/anchor';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import { Tree, IDL } from '../../../public/programs/tree';
import Router from 'next/router';

type cultivarAccount = {
	name: string;
	address: PublicKey|null;
	count: anchor.BN;
	initHeight: anchor.BN;
	initWidth: anchor.BN;
	scarcityPoints: anchor.BN;
	isInitialized: boolean;
};
type inputs = {
	name: string;
	initHeight: anchor.BN;
	initWidth: anchor.BN;
};

const CreateCultivar = () => {
const [input, setInput] = useState<inputs>({
	name: "",
	initWidth: 0,
	initHeight: 0
});
const [cultivar, setCultivar] = useState<cultivarAccount>({
	name: '',
	address: null,
	count: 0,
	scarcityPoints: 0 ,
	initWidth: 0,
	initHeight: 0,
	isInitialized: false,
});
const w = useAnchorWallet();

const connection = new Connection('https://api.devnet.solana.com');

const provider = new AnchorProvider(connection, w as Wallet, {
	commitment: 'confirmed',
});

const farmProgram = new PublicKey('6DDP3hohHprxPNUWVtwpK89QAzcB27Fk4NSCgcq368P6');

const programID = new PublicKey('EfYywm823JAajvTAHFv7wnKGi8M4R7BwqufaUEECxUxG');

const program = new Program(IDL, programID, provider);
let payer = program.provider;


const handleCreate = async () => {
	
	try {
		if (payer.publicKey) {
			// farm
			let [farm] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('farm')],
				farmProgram
				);

				console.log("farm", farm.toString());
			

			// cultivar_meta
			 let [cultivarMeta] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('cultivarmeta'), farm.toBuffer()],
				farmProgram
			);

			console.log('cultivarMeta', cultivarMeta.toString());
			
		let [cultivar] = anchor.web3.PublicKey.findProgramAddressSync(
				[
					Buffer.from('cultivar'),
					cultivarMeta.toBuffer(),
					Buffer.from(input.name),
				],
				program.programId
			);

			

			console.log('cultivar', cultivar.toString());

			let [fruitMint] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('fruitmint'), Buffer.from(input.name)],
				program.programId
			);
			console.log('fruitMint', fruitMint.toString());

			let [fruitMintAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('fruitmintauthority')],
				program.programId
			);

			console.log('fruitMintauthority', fruitMintAuthority.toString());
	  
			let cultivarState;

			cultivarState = await program.account.cultivar.fetchNullable(cultivar);

			if (cultivarState) {
				alert('cultivar already exists');
				return
			}	

		  console.log(
				'The inputs are ' + input.name + input.initHeight,
				+ input.initWidth
			);
			let tx = await program.methods
					.createCultivar(input.name.trim(), new anchor.BN(input.initHeight), new anchor.BN(input.initWidth))
					.accounts({
						farm,
						cultivarMeta,
						cultivar,	
						fruitMint,
						fruitMintAuthority,
						farmProgram,
					})
					.rpc();

				
			console.log('create cultivar transaction', tx);
	
			setTimeout(async () => {  }, 15000);
			cultivarState = await program.account.cultivar.fetchNullable(cultivar);	
		
			if (cultivarState) {
				console.log('the cultivar  is,', cultivarState);
				alert(`The Cultivar Is Initialized,
					 ${cultivarState},
					`
				);
			}	
			setInput({name:"",initHeight: 0 , initWidth:0}) ;
			alert("Success") ;

		} else {
			throw 'No pubkey provided';
		}
	} catch (e) {
		console.log(e);
	}
};

  return (
		<div>
			<Box
				component='form'
				className={styles.box}
				noValidate
				autoComplete='off'
				sx={{
					'& .MuiTextField-root': { m: 2 },
				}}
			>
				<Typography component='h3'>Enter your cultivar parameters</Typography>
				<div>
					<TextField
						color='info'
						id='name'
						label='Name'
						value={input.name}
						onChange={(e) => {
							setInput({ ...input, name: e.target.value });
						}}
						fullWidth
						required
					/>
				</div>
				<div>
					<TextField
						color='info'
						id='initial-width'
						label='Initial Width'
						value={input.initWidth}
						type='number'
						onChange={(e) => {
							setInput({ ...input, initWidth: e.target.value });
						}}
						fullWidth
						required
					/>
				</div>
				<div>
					<TextField
						color='info'
						id='initial-height'
						label='Initial Height'
						value={input.initHeight}
						onChange={(e) => {
							setInput({ ...input, initHeight: e.target.value });
						}}
						type='number'
						fullWidth
						required
					/>
				</div>
				<Button
					onClick={(e) => {
						e.preventDefault;
						handleCreate();
					}}
				>
					Enter Name
				</Button>
			</Box>
		</div>
	);
}

export default CreateCultivar